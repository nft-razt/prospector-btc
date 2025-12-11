import createMiddleware from 'next-intl/middleware';
// CORRECCIÓN: Usar alias absoluto
import { routing } from '@/lib/schemas/routing';

export const i18nHandler = createMiddleware(routing);
