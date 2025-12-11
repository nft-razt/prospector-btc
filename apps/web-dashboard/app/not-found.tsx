import { enDictionary } from '@prospector/i18n-config'; // Asegúrate de que el alias funcione o usa ruta relativa
// Si el alias no está configurado en tsconfig.base.json para la lib nueva, usa ruta relativa:
// import { enDictionary } from '../../../libs/shared/i18n-config/src/lib/dictionaries/en';
import { NotFoundScreen } from '@/components/system/not-found-screen';

// Importamos estilos globales explícitamente porque esta página reemplaza al RootLayout en caso de fallo catastrófico
import './global.css';

export default function GlobalNotFound() {
  const texts = enDictionary.NotFound;

  return (
    <html lang="en">
      <body>
        <NotFoundScreen
          texts={texts}
          redirectPath="/"
        />
      </body>
    </html>
  );
}
