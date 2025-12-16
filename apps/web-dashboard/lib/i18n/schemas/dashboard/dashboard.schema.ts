import { z } from "zod";

export const DashboardSchema = z.object({
  sidebar: z.object({
    overview: z.string(),
    network: z.string(),
    wallet_lab: z.string(),
    academy: z.string(),
    settings: z.string(),
  }),
  header: z.object({
    welcome: z.string(),
    status_online: z.string(),
  }),
  user_nav: z.object({
    profile: z.string(),
    billing: z.string(),
    settings: z.string(),
    logout: z.string(),
  }),
  fleet: z.object({
    title: z.string(),
    live_feed: z.string(),
    no_signal: z.string(),
    deploy_hint: z.string(),
    connection_lost: z.string(),
  }),
});

export type DashboardParams = z.infer<typeof DashboardSchema>;
