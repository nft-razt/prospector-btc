import { useTranslations } from "next-intl";
import { NotFoundScreen } from "@/components/system/not-found-screen";

export default function LocalizedNotFound() {
  const t = useTranslations("NotFound");

  return (
    <NotFoundScreen
      texts={{
        title: t("title"),
        description: t("description"),
        error_code: t("error_code"),
        cta_return: t("cta_return"),
      }}
      redirectPath="/dashboard" // O "/" según preferencia lógica
    />
  );
}
