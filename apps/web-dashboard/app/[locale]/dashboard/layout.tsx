import { redirect } from "next/navigation";
import { auth } from "@/lib/auth/config";
import { Sidebar } from "@/components/layout/sidebar";
import { TopNav } from "@/components/layout/top-nav";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export default async function DashboardLayout({ children }: DashboardLayoutProps) {
  // 🔐 Security Checkpoint
  const session = await auth();

  if (!session || !session.user) {
    redirect("/login");
  }

  return (
    <div className="flex h-screen w-full bg-background overflow-hidden transition-colors duration-300">
      {/* 1. SIDEBAR */}
      <aside className="hidden md:flex w-64 flex-col border-r border-border bg-card/50 backdrop-blur-xl h-full z-20">
        <Sidebar />
      </aside>

      {/* 2. ZONA DE CONTENIDO */}
      <div className="flex flex-1 flex-col h-full overflow-hidden relative">
        <header className="h-16 border-b border-border bg-background/80 backdrop-blur-md flex items-center px-6 justify-between z-10 sticky top-0">
          <TopNav user={session.user} />
        </header>

        <main className="flex-1 overflow-y-auto p-6 scrollbar-thin scrollbar-thumb-primary/20 scrollbar-track-transparent">
          <div className="max-w-7xl mx-auto space-y-6 animate-in fade-in zoom-in-95 duration-500">
            {children}
          </div>

          <footer className="py-8 text-center text-[10px] text-muted-foreground font-mono opacity-50 hover:opacity-100 transition-opacity">
            PROSPECTOR SUITE v4.0 // HYDRA ARCHITECTURE // ENCRYPTED UPLINK
          </footer>
        </main>
      </div>
    </div>
  );
}
