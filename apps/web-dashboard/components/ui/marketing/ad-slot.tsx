"use client";

export function AdSlot() {
  return (
    <div className="w-full h-[90px] bg-[#0A0A0A] border-b border-zinc-800 flex items-center justify-center relative overflow-hidden">
      {/* Pattern Background */}
      <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5"></div>

      <div className="z-10 text-center">
        {/* Aquí iría el script de Google AdSense */}
        {/* <ins className="adsbygoogle" ... /> */}

        {/* Placeholder mientras no hay script */}
        <div className="text-zinc-600 text-[10px] font-mono uppercase tracking-widest border border-zinc-800 px-4 py-2 rounded bg-black/50">
          Sponsored Slot (728x90)
        </div>
      </div>
    </div>
  );
}
