'use client';

import { Link } from '@/lib/schemas/routing';
import { useTranslations } from 'next-intl';
import { Button } from '@/components/ui/kit/button';
import { Cpu } from 'lucide-react';

export function Header() {
  const t = useTranslations('Landing');

  return (
    <header className="fixed top-0 left-0 right-0 z-50 border-b border-white/10 bg-black/50 backdrop-blur-xl h-16 transition-all duration-300">
      <div className="mx-auto flex h-full max-w-7xl items-center justify-between px-6">
        <Link href="/" className="flex items-center gap-2 group">
          <div className="h-8 w-8 bg-primary/10 rounded-lg border border-primary/20 flex items-center justify-center group-hover:bg-primary/20 transition-colors">
             <Cpu className="w-5 h-5 text-primary" />
          </div>
          <span className="text-lg font-bold tracking-tighter text-white font-mono">
            PROSPECTOR <span className="text-primary">BTC</span>
          </span>
        </Link>

        <div className="flex gap-4">
          <Link href="/login">
            <Button variant="ghost" size="sm" className="text-zinc-400 hover:text-white font-mono uppercase tracking-wider text-xs">
              Operator Login
            </Button>
          </Link>
        </div>
      </div>
    </header>
  );
}
