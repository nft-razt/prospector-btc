import { type SystemParams } from "../../../schemas/system/system.schema";

export const systemContent = {
  not_found: {
    title: "SIGNAL LOST",
    description:
      "The requested coordinates do not correspond to any known sector in the Prospector network.",
    error_code: "ERR_404_VOID",
    cta_return: "Return to Command Center",
  },
  maintenance: {
    title: "SYSTEM UPDATING",
    message: "Hydra-Zero protocol is undergoing critical maintenance.",
  },
} satisfies SystemParams;
