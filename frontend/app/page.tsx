"use client";

import { CorbadoAuth } from "@corbado/react";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const router = useRouter();

  const onLoggedIn = () => {
    router.push("/profile");
  };

  return (
    <div className="login-container">
      <CorbadoAuth onLoggedIn={onLoggedIn} />
    </div>
  );
}
