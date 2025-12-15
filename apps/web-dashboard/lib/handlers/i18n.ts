import createMiddleware from "next-intl/middleware";
import { routing } from "@/lib/schemas/routing"; // ✅ Ruta absoluta

export const i18nHandler = createMiddleware(routing);
