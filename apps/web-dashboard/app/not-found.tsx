// ✅ Import local desde la estructura migrada
import { enDictionary } from "@/lib/i18n-source/dictionaries/en";
import { NotFoundScreen } from "@/components/system/not-found-screen";
import "./global.css";

export default function GlobalNotFound() {
  // Acceso directo tipado al diccionario local
  const texts = enDictionary.System.not_found;

  return (
    <html lang="en">
      <body>
        <NotFoundScreen texts={texts} redirectPath="/" />
      </body>
    </html>
  );
}
