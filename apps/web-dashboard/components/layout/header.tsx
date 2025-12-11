'use client';

import { Link } from '@/lib/schemas/routing';
import { useTranslations } from 'next-intl';
import { Button } from '@/components/ui/kit/button';

export function Header() {
  const t = useTranslations('Landing.hero');

  return (
    <header className="fixed top-0 left-0 right-0 z-50 border-b border-white/10 bg-black/50 backdrop-blur-xl">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-6">
        <Link href="/" className="text-xl font-bold tracking-tighter text-white">
          PROSPECTOR <span className="text-emerald-500">BTC</span>
        </Link>

        <div className="flex gap-4">
          <Link href="/login">
            <Button variant="ghost" className="text-zinc-400 hover:text-white">
              Login
            </Button>
          </Link>
        </div>
      </div>
    </header>
  );
}
