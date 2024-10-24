import "./globals.css"
import type {Metadata} from "next"
import {Inter} from "next/font/google"
import Provider from "@/app/_utils/Provider"

const inter = Inter({subsets: ["latin"]})

export const metadata: Metadata = {
    title: "Polling Application",
    description: "Go passwordless with Corbado and Next.js",
}

export default function RootLayout(
    {
        children,
    }: {
        children: React.ReactNode
    }) {
    return (
        <html lang='en'>
        <Provider>
            <body className={inter.className}>{children}</body>
        </Provider>
        </html>
    )
}
