import { z } from "zod";

export const AuthSchema = z.object({
  login: z.object({
    title: z.string(),
    google_btn: z.string(),
    footer_text: z.string(),
  }),
  logout: z.object({
    label: z.string(),
    confirm_msg: z.string(),
  }),
  errors: z.object({
    signin_failed: z.string(),
    access_denied: z.string(),
  }),
});

export type AuthParams = z.infer<typeof AuthSchema>;
