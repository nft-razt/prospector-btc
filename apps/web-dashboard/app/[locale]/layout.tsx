import { NextIntlClientProvider } from 'next-intl';
import { getMessages } from 'next-intl/server';
import { notFound } from 'next/navigation';
import { routing } from '@/lib/schemas/routing';
import Providers from '../../providers';
// Importamos CSS aquí también para asegurar que cargue cuando la ruta [locale] reemplaza al root
import '../../global.css';

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
  if (!routing.locales.includes(locale as any)) {
    notFound();
  }

  const messages = await getMessages();

  return (
    <html lang={locale} className="dark" suppressHydrationWarning>
      <body className="bg-[#050505] text-slate-200 antialiased min-h-screen">
        <NextIntlClientProvider messages={messages}>
          <Providers>
            {children}
          </Providers>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
