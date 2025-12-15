"use client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useState } from "react";
import { Toaster } from "sonner"; // 👈 Importación
import { ThemeProvider } from "@/components/providers/theme-provider";

export default function Providers({ children }: { children: React.ReactNode }) {
  // Garantizamos que el cliente se crea una sola vez por sesión
  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            // Fail fast en desarrollo, reintentos en producción
            retry: process.env.NODE_ENV === "production" ? 3 : 1,
            refetchOnWindowFocus: false,
          },
        },
      }),
  );

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider
        attribute="class"
        defaultTheme="dark"
        enableSystem
        disableTransitionOnChange
      >
        {children}

        {/* 🔔 SISTEMA DE NOTIFICACIONES GLOBAL */}
        <Toaster
          position="bottom-right"
          theme="dark"
          className="font-mono"
          toastOptions={{
            classNames: {
              toast:
                "bg-[#0a0a0a] border border-slate-800 text-slate-200 shadow-2xl",
              title: "text-emerald-500 font-bold tracking-wide",
              description: "text-slate-400 text-xs",
              actionButton: "bg-emerald-600 text-white",
              cancelButton: "bg-slate-800 text-slate-400",
              error: "border-red-900/50 bg-red-950/10 text-red-400",
              success: "border-emerald-900/50 bg-emerald-950/10",
            },
          }}
        />
      </ThemeProvider>
    </QueryClientProvider>
  );
}
