export function Footer() {
  return (
    <footer className="border-t border-white/5 bg-black py-12 mt-auto relative z-10">
      <div className="mx-auto max-w-7xl px-6 flex flex-col items-center justify-center gap-6">
        {/* STATUS INDICATOR */}
        <div className="flex items-center gap-4">
          <div className="h-px w-12 bg-gradient-to-r from-transparent to-zinc-800" />
          <div className="flex items-center gap-2 px-3 py-1 bg-zinc-900/50 rounded border border-zinc-800">
            <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" />
            <span className="text-[10px] text-zinc-400 font-mono uppercase tracking-widest">
              Nodes Active
            </span>
          </div>
          <div className="h-px w-12 bg-gradient-to-l from-transparent to-zinc-800" />
        </div>

        <p className="text-[10px] text-zinc-600 font-mono text-center max-w-md leading-relaxed">
          PROSPECTOR SUITE v4.0 // ACADEMIC RESEARCH PROJECT
          <br />
          TARGETING SECP256K1 ENTROPY CLUSTERS
        </p>

        <p className="text-[10px] text-zinc-700 font-mono">
          © 2025 MIT LICENSE
        </p>
      </div>
    </footer>
  );
}
