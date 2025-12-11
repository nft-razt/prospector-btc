export function Footer() {
  return (
    <footer className="border-t border-white/10 bg-black py-8 mt-auto">
      <div className="mx-auto max-w-7xl px-6 text-center">
        <div className="flex justify-center items-center gap-4 mb-4">
            <div className="h-px w-8 bg-zinc-800" />
            <span className="text-[10px] text-zinc-600 font-mono uppercase tracking-widest">
                System Status: Operational
            </span>
            <div className="h-px w-8 bg-zinc-800" />
        </div>
        <p className="text-[10px] text-zinc-500 font-mono">
          © 2025 PROSPECTOR SUITE // ACADEMIC RESEARCH // MIT LICENSE
        </p>
      </div>
    </footer>
  );
}
