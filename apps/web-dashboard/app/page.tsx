import { redirect } from 'next/navigation';

// Esta página es invisible. Su única función es capturar "/"
// y lanzarlo a "/en" (o el idioma detectado por middleware).
export default function RootPage() {
  redirect('/en');
}
