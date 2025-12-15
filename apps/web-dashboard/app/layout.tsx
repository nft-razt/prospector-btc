import "./global.css";

export const metadata = {
  title: "Prospector System",
  description: "Hydra-Zero Node Network",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-[#050505] text-white antialiased">{children}</body>
    </html>
  );
}
