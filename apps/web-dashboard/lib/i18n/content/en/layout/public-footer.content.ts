import { type PublicFooterParams } from "../../../schemas/layout/public-footer.schema";

export const publicFooterContent = {
  copyright: "© 2025 Prospector Research Group. MIT License.",
  links: {
    privacy: { label: "Data Privacy", href: "/privacy" },
    terms: { label: "Protocol Terms", href: "/terms" },
    github: { label: "Source Code", href: "https://github.com/prospector-btc" },
    documentation: { label: "Docs", href: "/docs" },
  },
  disclaimer:
    "Academic research tool. Not designed for illegal use. Use responsibly.",
  status: {
    label: "System Status",
    operational: "All Systems Normal",
    degraded: "Performance Degraded",
  },
} satisfies PublicFooterParams;
