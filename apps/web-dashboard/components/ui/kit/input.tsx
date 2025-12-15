import * as React from "react";
import { cn } from "@/lib/utils/cn";

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  /**
   * Indica si el campo tiene un error de validación.
   * Cambia el borde a color destructivo.
   */
  hasError?: boolean;
}

/**
 * APARATO: INPUT
 * Campo de entrada de texto optimizado para interfaces de datos densos.
 * Estilo 'Mono' para alineación perfecta de caracteres en credenciales/hashes.
 */
const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, hasError, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "flex h-10 w-full rounded-md border bg-black/50 px-3 py-2 text-sm font-mono shadow-sm transition-colors",
          "file:border-0 file:bg-transparent file:text-sm file:font-medium",
          "placeholder:text-muted-foreground",
          "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
          // Lógica condicional de estilos
          hasError
            ? "border-destructive focus-visible:ring-destructive text-destructive placeholder:text-destructive/50"
            : "border-input focus-visible:border-primary/50",
          className,
        )}
        ref={ref}
        {...props}
      />
    );
  },
);
Input.displayName = "Input";

export { Input };
