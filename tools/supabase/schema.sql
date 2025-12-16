-- =================================================================
-- PROSPECTOR STRATEGIC ENGINE SCHEMA (SUPABASE)
-- ARQUITECTURA: MULTI-TENANT + ZERO KNOWLEDGE VAULT
-- =================================================================

-- 1. EXTENSIONES Y CONFIGURACIÓN
create extension if not exists "uuid-ossp";
create extension if not exists "pgcrypto";

-- -----------------------------------------------------------------
-- 2. IDENTIDAD Y PERFILES
-- Extiende la tabla auth.users nativa de Supabase
-- -----------------------------------------------------------------
create table public.profiles (
  id uuid references auth.users on delete cascade not null primary key,
  email text unique not null,
  full_name text,
  avatar_url text,
  created_at timestamp with time zone default timezone('utc'::text, now()) not null,
  updated_at timestamp with time zone default timezone('utc'::text, now()) not null
);

-- -----------------------------------------------------------------
-- 3. ESPACIOS DE TRABAJO (TENANCY)
-- Unidad fundamental de agrupación. Los datos pertenecen al Workspace.
-- -----------------------------------------------------------------
create table public.workspaces (
  id uuid default uuid_generate_v4() primary key,
  name text not null,
  slug text unique not null, -- Para URLs amigables
  owner_id uuid references public.profiles(id) not null,
  plan_tier text default 'observer', -- 'observer', 'operator', 'architect'
  credits_balance bigint default 0,
  created_at timestamp with time zone default timezone('utc'::text, now()) not null
);

-- Relación N:M entre Usuarios y Workspaces con Roles
create table public.workspace_members (
  workspace_id uuid references public.workspaces(id) on delete cascade not null,
  user_id uuid references public.profiles(id) on delete cascade not null,
  role text default 'member', -- 'owner', 'admin', 'member', 'viewer'
  joined_at timestamp with time zone default timezone('utc'::text, now()) not null,
  primary key (workspace_id, user_id)
);

-- -----------------------------------------------------------------
-- 4. ARCHIVO HISTÓRICO (Cold Storage)
-- Datos migrados desde Turso al finalizar un Job.
-- -----------------------------------------------------------------
create table public.archived_jobs (
  id uuid default uuid_generate_v4() primary key,
  workspace_id uuid references public.workspaces(id) not null,

  -- Metadatos del Job Original (Turso)
  original_job_id text not null,
  range_start text not null, -- Padded String U256
  range_end text not null,   -- Padded String U256
  strategy_type text not null,

  -- Estadísticas de Ejecución
  total_hashes bigint default 0,
  duration_seconds int default 0,
  findings_count int default 0,

  created_at timestamp with time zone default timezone('utc'::text, now()) not null
);

-- -----------------------------------------------------------------
-- 5. BÓVEDA ACORAZADA (The Vault)
-- Almacena hallazgos sensibles.
-- IMPORTANTE: 'encrypted_data' es un blob cifrado en el CLIENTE (AES-GCM).
-- El servidor NO tiene la clave para descifrar esto.
-- -----------------------------------------------------------------
create table public.vault_items (
  id uuid default uuid_generate_v4() primary key,
  workspace_id uuid references public.workspaces(id) not null,

  -- Datos Públicos (Metadatos para búsqueda)
  label text not null, -- Ej: "Posible Satoshi Era Wallet"
  public_address text not null,
  wallet_type text not null,

  -- Datos Clasificados (Solo descifrables por el usuario)
  -- Contiene JSON cifrado: { wif: "...", mnemonic: "...", path: "..." }
  encrypted_blob text not null,
  iv text not null, -- Vector de Inicialización (necesario para AES)
  auth_tag text not null, -- Tag de autenticación GCM

  created_at timestamp with time zone default timezone('utc'::text, now()) not null
);

-- =================================================================
-- 6. SEGURIDAD (ROW LEVEL SECURITY - RLS)
-- =================================================================

alter table public.profiles enable row level security;
alter table public.workspaces enable row level security;
alter table public.workspace_members enable row level security;
alter table public.archived_jobs enable row level security;
alter table public.vault_items enable row level security;

-- Helper Function: ¿Tiene el usuario acceso al workspace?
create or replace function public.is_workspace_member(_workspace_id uuid)
returns boolean as $$
begin
  return exists (
    select 1 from public.workspace_members
    where workspace_id = _workspace_id
    and user_id = auth.uid()
  );
end;
$$ language plpgsql security definer;

-- Políticas Profiles
create policy "Users can view own profile" on public.profiles
  for select using (auth.uid() = id);
create policy "Users can update own profile" on public.profiles
  for update using (auth.uid() = id);

-- Políticas Workspaces
create policy "Members can view workspace" on public.workspaces
  for select using (
    auth.uid() = owner_id OR
    exists (select 1 from public.workspace_members where workspace_id = id and user_id = auth.uid())
  );

-- Políticas Vault (CRÍTICO)
create policy "Members can view vault items" on public.vault_items
  for select using (public.is_workspace_member(workspace_id));

create policy "Members can insert vault items" on public.vault_items
  for insert with check (public.is_workspace_member(workspace_id));

-- =================================================================
-- 7. AUTOMATIZACIÓN (TRIGGERS)
-- =================================================================

-- Función: Crear perfil y workspace personal al registrarse
create or replace function public.handle_new_user()
returns trigger as $$
declare
  new_workspace_id uuid;
begin
  -- 1. Insertar Perfil
  insert into public.profiles (id, email, full_name, avatar_url)
  values (new.id, new.email, new.raw_user_meta_data->>'full_name', new.raw_user_meta_data->>'avatar_url');

  -- 2. Crear Workspace Personal
  insert into public.workspaces (name, slug, owner_id, plan_tier)
  values ('Personal Space', 'personal-' || substr(new.id::text, 1, 8), new.id, 'observer')
  returning id into new_workspace_id;

  -- 3. Vincular Miembro
  insert into public.workspace_members (workspace_id, user_id, role)
  values (new_workspace_id, new.id, 'owner');

  return new;
end;
$$ language plpgsql security definer;

-- Trigger de Auth
create trigger on_auth_user_created
  after insert on auth.users
  for each row execute procedure public.handle_new_user();
