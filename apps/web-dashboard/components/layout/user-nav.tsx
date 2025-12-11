'use client';

import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from '@radix-ui/react-avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@radix-ui/react-dropdown-menu';
import { useTranslations } from 'next-intl';
import { signOut } from 'next-auth/react';
import { LogOut, Settings, CreditCard, User, ShieldAlert } from 'lucide-react';
import { useHeimdall } from '@/hooks/use-heimdall';
import { cn } from '@/lib/utils/cn';

interface UserNavProps {
  user: {
    name?: string | null;
    email?: string | null;
    image?: string | null;
  }
}

/**
 * ORGANISMO: USER NAV
 * Menú contextual del perfil de usuario con acciones de sesión.
 * Integrado con NextAuth y Logging de seguridad.
 */
export function UserNav({ user }: UserNavProps) {
  const t = useTranslations('Dashboard.user_nav');
  const logger = useHeimdall('UserNav');

  // Lógica de fallback para avatar (Iniciales)
  const initials = user.name
    ?.split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2) || 'OP';

  const handleSignOut = async () => {
    logger.info(`Cerrando sesión de usuario: ${user.email}`);
    await signOut();
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button className="relative h-9 w-9 rounded-full focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 focus:ring-offset-black transition-all hover:scale-105 active:scale-95">
          <Avatar className="h-9 w-9 rounded-full overflow-hidden border border-input bg-muted">
            <AvatarImage
              src={user.image || ''}
              alt={user.name || 'Operator'}
              className="object-cover h-full w-full"
            />
            <AvatarFallback className="flex h-full w-full items-center justify-center bg-zinc-900 text-xs font-bold text-primary font-mono">
              {initials}
            </AvatarFallback>
          </Avatar>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="w-64 bg-[#0a0a0a]/95 backdrop-blur-xl border border-zinc-800 text-zinc-200 p-1 shadow-2xl rounded-xl animate-in fade-in zoom-in-95 slide-in-from-top-2"
        align="end"
        forceMount
      >
        <DropdownMenuLabel className="font-normal p-3">
          <div className="flex flex-col space-y-1">
            <p className="text-sm font-medium leading-none text-white tracking-wide">
              {user.name}
            </p>
            <p className="text-xs leading-none text-zinc-500 font-mono truncate">
              {user.email}
            </p>
          </div>
        </DropdownMenuLabel>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-1" />

        <DropdownMenuGroup>
          <DropdownMenuItem className="cursor-pointer hover:bg-zinc-800/80 hover:text-white focus:bg-zinc-800/80 px-2 py-2 rounded-md flex gap-3 text-xs items-center transition-colors outline-none group">
             <div className="p-1 bg-zinc-900 rounded group-hover:bg-black transition-colors">
               <User className="w-3.5 h-3.5 text-zinc-400 group-hover:text-primary" />
             </div>
             {t('profile')}
          </DropdownMenuItem>

          <DropdownMenuItem className="cursor-pointer hover:bg-zinc-800/80 hover:text-white focus:bg-zinc-800/80 px-2 py-2 rounded-md flex gap-3 text-xs items-center transition-colors outline-none group">
             <div className="p-1 bg-zinc-900 rounded group-hover:bg-black transition-colors">
                <CreditCard className="w-3.5 h-3.5 text-zinc-400 group-hover:text-primary" />
             </div>
             {t('billing')}
          </DropdownMenuItem>

          <DropdownMenuItem className="cursor-pointer hover:bg-zinc-800/80 hover:text-white focus:bg-zinc-800/80 px-2 py-2 rounded-md flex gap-3 text-xs items-center transition-colors outline-none group">
             <div className="p-1 bg-zinc-900 rounded group-hover:bg-black transition-colors">
                <Settings className="w-3.5 h-3.5 text-zinc-400 group-hover:text-primary" />
             </div>
             {t('settings')}
          </DropdownMenuItem>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="h-px bg-zinc-800 my-1" />

        {/* Zona de Peligro / Logout */}
        <DropdownMenuItem
            className="cursor-pointer hover:bg-red-950/30 text-red-400 hover:text-red-300 focus:bg-red-950/30 px-2 py-2 rounded-md flex gap-3 text-xs items-center transition-colors outline-none group"
            onClick={handleSignOut}
        >
          <div className="p-1 bg-red-950/20 rounded group-hover:bg-red-900/40 transition-colors">
            <LogOut className="w-3.5 h-3.5" />
          </div>
          <span className="font-medium tracking-wide">{t('logout')}</span>
        </DropdownMenuItem>

        {/* Footer Técnico */}
        <div className="px-2 py-2 mt-1 bg-black/40 rounded border border-zinc-900 flex items-center justify-between text-[9px] text-zinc-600 font-mono">
           <span className="flex items-center gap-1">
             <ShieldAlert className="w-2.5 h-2.5" /> SECURE
           </span>
           <span>TLS 1.3</span>
        </div>

      </DropdownMenuContent>
    </DropdownMenu>
  );
}
