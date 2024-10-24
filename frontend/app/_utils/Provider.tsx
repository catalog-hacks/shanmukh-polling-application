"use client";

import { CorbadoProvider } from "@corbado/react";
import { UserProvider } from "./UserContext";
export default function Provider({ children }: { children: React.ReactNode }) {
  return (
    <CorbadoProvider
      projectId={process.env.NEXT_PUBLIC_CORBADO_PROJECT_ID!}
      darkMode="off"
      setShortSessionCookie={true}
    >
      <UserProvider>{children}</UserProvider>
    </CorbadoProvider>
  );
}
