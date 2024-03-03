import type { Metadata } from "next";
import { Noto_Sans as Noto } from "next/font/google";
import "./globals.css";

const noto = Noto({ subsets: ["latin-ext"] });

export const metadata: Metadata = {
  title: "Orbitus",
  description: "Orbitus cloud",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={noto.className}>{children}</body>
    </html>
  );
}
