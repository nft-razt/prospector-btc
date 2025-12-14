/**
 * =================================================================
 * APARATO: MASTER DASHBOARD LAYET (THE SHELL)
 * CLASIFICACIÓN: ARQUITECTURA VISUAL
 * RESPONSABILIDAD: FRAMEWORK DE PERSISTENCIA UX & SEGURIDAD
 * =================================================================
 */

import { redirect } from "next/navigation";
import { auth } from "@/lib/auth/config";
import { Sidebar } from "@/components/layout/sidebar";
import { TopNav } from "@/components/layout/top-nav";

export default async function DashboardLayout({
  children
}: {
  children: React.ReactNode
}) {
  // 🔐 CONTROL DE ACCESO SOBERANO
  const session = await auth();
  if (!session || !session.user) {
    redirect("/login");
  }

  return (
    <div className="flex h-screen w-full bg-[#050505] text-slate-200 overflow-hidden font-sans">

      {/* CAPA DE RUIDO VISUAL (TEXTURE) */}
      <div className="fixed inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] pointer-events-none z-50" />

      {/* CAPA DE GRADIENTE ATMOSFÉRICO */}
      <div className="fixed top-0 left-0 w-full h-full bg-linear-to-br from-primary/5 via-transparent to-purple-500/5 pointer-events-none" />

      {/* 1. SIDEBAR (Navegación & Health) */}
      <aside className="hidden md:flex w-72 flex-col z-30 relative">
        <Sidebar />
      </aside>

      {/* 2. MAIN VIEWPORT */}
      <div className="flex flex-1 flex-col relative overflow-hidden h-full">

        {/* HEADER ESTRATÉGICO */}
        <header className="h-16 border-b border-white/5 bg-black/20 backdrop-blur-md flex items-center px-6 z-20">
          <TopNav user={session.user} />
        </header>

        {/* AREA DE TRABAJO SCROLLABLE */}
        <main className="flex-1 overflow-y-auto relative custom-scrollbar">
          {/* Contenedor de contenido con máximo ancho para legibilidad */}
          <div className="max-w-7xl mx-auto p-8 min-h-full">
            {children}
          </div>

          {/* FOOTER TÉCNICO FLOTANTE */}
          <footer className="p-8 mt-auto border-t border-white/5 bg-black/40">
            <div className="flex justify-between items-center opacity-30 hover:opacity-100 transition-opacity duration-500">
               <span className="text-[9px] font-mono tracking-[0.4em] uppercase">
                 Prospector // Distributed Intelligence System
               </span>
               <span className="text-[9px] font-mono">
                 v4.5.0-ALPHA // SECP256K1_AUDIT_READY
               </span>
            </div>
          </footer>
        </main>
      </div>
    </div>
  );
}
