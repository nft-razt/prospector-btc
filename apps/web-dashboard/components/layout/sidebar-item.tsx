"use client";

import { motion } from "framer-motion";
import { Link } from "@/lib/schemas/routing";
import { cn } from "@/lib/utils/cn";
import { type RouteItem } from "@/config/navigation";
import { useHeimdall } from "@/hooks/use-heimdall";

interface SidebarItemProps {
  item: RouteItem;
  isActive: boolean;
  label: string;
}

/**
 * ÁTOMO: SIDEBAR ITEM
 * Maneja la presentación y el logging de navegación de un ítem individual.
 */
export function SidebarItem({ item, isActive, label }: SidebarItemProps) {
  const logger = useHeimdall("SidebarNavigation");

  const handleClick = () => {
    logger.debug(`Navegando a: ${item.href}`);
  };

  return (
    <Link
      href={item.href}
      onClick={handleClick}
      className={cn(
        "group flex items-center px-3 py-2.5 text-sm font-medium rounded-md transition-all relative overflow-hidden outline-none focus-visible:ring-2 focus-visible:ring-primary/50",
        isActive
          ? "text-primary bg-primary/10"
          : "text-muted-foreground hover:text-foreground hover:bg-muted/50",
      )}
    >
      {/* Indicador Activo Animado */}
      {isActive && (
        <motion.div
          layoutId="sidebar-active-pill"
          className="absolute left-0 top-0 bottom-0 w-1 bg-primary rounded-r-full shadow-[0_0_10px_rgba(16,185,129,0.5)]"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.2 }}
        />
      )}

      <item.icon
        className={cn(
          "mr-3 h-5 w-5 transition-colors duration-200",
          isActive
            ? "text-primary drop-shadow-md"
            : "text-muted-foreground group-hover:text-foreground",
        )}
      />
      <span className="relative z-10 font-medium tracking-wide">{label}</span>
    </Link>
  );
}
