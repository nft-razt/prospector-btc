import { NextIntlClientProvider } from 'next-intl';
import { getMessages } from 'next-intl/server';
import { notFound } from 'next/navigation';
import { routing } from '@/lib/schemas/routing';
import Providers from '../../providers'; // Ajusta la ruta relativa si es necesario (está en ../../)
import '../../global.css'; // 🔥 IMPORTACIÓN CRÍTICA DE ESTILOS

// Metadatos globales
export const metadata = {
  title: 'Prospector // Mission Control',
  description: 'Distributed Entropy Audit System',
};

export default async function LocaleLayout({
  children,
  params: { locale }
}: {
  children: React.ReactNode;
  params: { locale: string };
}) {
  // Validación de seguridad para el locale
  if (!routing.locales.includes(locale as any)) {
    notFound();
  }

  const messages = await getMessages();

  return (
    <html lang={locale} suppressHydrationWarning>
      <body className="bg-[#050505] text-slate-200 antialiased min-h-screen selection:bg-emerald-500/30">
        <NextIntlClientProvider messages={messages}>
          <Providers>
             {children}
          </Providers>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
