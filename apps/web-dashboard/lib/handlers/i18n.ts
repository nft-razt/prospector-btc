import createMiddleware from 'next-intl/middleware';
import { routing } from '../../i18n/routing'; // Asegúrate de que esta ruta apunte a tu routing.ts

export const i18nHandler = createMiddleware(routing);
