import { type AuthParams } from "../../../schemas/auth/auth.schema";

export const authContent = {
  login: {
    title: "Identify Yourself",
    google_btn: "Authenticate via Google",
    footer_text: "Secure Connection // TLS 1.3",
  },
  logout: {
    label: "Logging out...",
    confirm_msg: "Session Terminated",
  },
  errors: {
    signin_failed: "Authentication Handshake Failed",
    access_denied: "Access Denied: Authorization Level Insufficient",
  },
} satisfies AuthParams;
