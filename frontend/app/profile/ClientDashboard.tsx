"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { useUser } from "../_utils/UserContext";

interface User {
  userId: string;
  fullName: string;
}

interface ClientDashboardProps {
  user: User;
  token: string;
}

export default function ClientDashboard({ user, token }: ClientDashboardProps) {
  const { setUser } = useUser();
  const router = useRouter();

  useEffect(() => {
    setUser(user);
    if (token) {
      localStorage.setItem("token", token);
    }
  }, [user, token, setUser]);

  return (
    <div className="p-6">
      <h2 className="text-2xl font-semibold">
        Welcome to your Dashboard, {user.fullName}!
      </h2>
    </div>
  );
}
