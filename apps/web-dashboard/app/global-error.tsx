"use client";

import { useEffect } from "react";

/**
 * APARATO: GLOBAL ERROR SHIELD
 * Última línea de defensa. Captura errores en el RootLayout.
 * Debe renderizar su propio <html> y <body>.
 */
export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    console.error("🔥 CATASTROPHIC FAILURE:", error);
  }, [error]);

  return (
    <html lang="en">
      <body className="bg-black text-white font-sans min-h-screen flex flex-col items-center justify-center p-4">
        <div className="max-w-md w-full text-center space-y-6">
          <h1 className="text-4xl font-bold text-red-600 tracking-tighter">
            CRITICAL FAILURE
          </h1>
          <p className="text-zinc-500 font-mono text-xs">
            CORE SYSTEMS UNRESPONSIVE. MANUAL RESET REQUIRED.
          </p>
          <div className="p-4 bg-zinc-900 rounded border border-zinc-800 text-left overflow-auto max-h-32">
            <code className="text-[10px] text-red-400">{error.message}</code>
          </div>
          <button
            onClick={() => reset()}
            className="px-6 py-3 bg-white text-black font-bold rounded uppercase tracking-widest hover:bg-zinc-200 transition-colors"
          >
            System Reset
          </button>
        </div>
      </body>
    </html>
  );
}
