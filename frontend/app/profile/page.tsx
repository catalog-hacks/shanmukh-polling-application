import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import getNodeSDK from "../_utils/nodeSdk";
import ClientDashboard from "./ClientDashboard";

export default async function ProfilePage() {
  const cookieStore = cookies();
  const session = cookieStore.get("cbo_short_session");

  if (!session) {
    redirect("/");
  }

  const sdk = getNodeSDK();
  let user;

  try {
    user = await sdk.sessions().validateToken(session.value);

    if (!user) {
      throw new Error("Invalid user session");
    }

    const response = await fetch("http://localhost:3030/api/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        user_id: user.userId,
        name: user.fullName,
      }),
    });

    const data = await response.json();

    if (data.token) {
      return <ClientDashboard user={user} token={data.token} />;
    }
  } catch (error) {
    console.error("Error during session validation or login:", error);
    redirect("/");
  }

  return null;
}
