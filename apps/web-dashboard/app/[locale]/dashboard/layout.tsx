import { redirect } from "next/navigation";
import { auth } from "@/lib/auth/config";
import { Sidebar } from "@/components/layout/sidebar";
import { TopNav } from "@/components/layout/top-nav";

export default async function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();
  if (!session || !session.user) {
    redirect("/login");
  }

  return (
    <div className="flex h-screen w-full overflow-hidden font-sans">
      {/* CAPA DE RUIDO VISUAL */}
      <div className="fixed inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-[0.03] pointer-events-none z-50" />

      {/* GRADIENTE DE FONDO */}
      <div className="fixed top-0 left-0 w-full h-full bg-gradient-to-br from-emerald-900/5 via-[#050505] to-purple-900/5 pointer-events-none -z-10" />

      {/* SIDEBAR */}
      <aside className="hidden md:flex w-72 flex-col z-30 relative border-r border-white/5 bg-black/40 backdrop-blur-xl">
        <Sidebar />
      </aside>

      {/* MAIN CONTENT */}
      <div className="flex flex-1 flex-col relative overflow-hidden h-full">
        <header className="h-16 border-b border-white/5 bg-black/20 backdrop-blur-md flex items-center px-6 z-20">
          <TopNav user={session.user} />
        </header>

        <main className="flex-1 overflow-y-auto relative custom-scrollbar p-8">
          <div className="max-w-7xl mx-auto min-h-full pb-12">{children}</div>

          <footer className="mt-auto border-t border-white/5 pt-6 text-center opacity-30 hover:opacity-100 transition-opacity">
            <span className="text-[9px] font-mono tracking-[0.4em] uppercase">
              Prospector // Hydra-Zero Protocol
            </span>
          </footer>
        </main>
      </div>
    </div>
  );
}
