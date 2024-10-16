import { cookies } from "next/headers";
import getNodeSDK from "@/app/_utils/nodeSdk";
import { redirect } from "next/navigation";
import ClientSideActions from "./component";

export default async function Profile() {
    const cookieStore = cookies();
    const session = cookieStore.get("cbo_short_session");

    if (!session) {
        return redirect("/");
    }

    const sdk = getNodeSDK();
    let user;

    try {
        user = await sdk.sessions().validateToken(session.value);
        if (!user) {
            throw Error;
        }

        await fetch("http://localhost:3030/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                user_id: user.userId,
                name: user.fullName,
            }),
        });

        const pollsResponse = await fetch(`http://localhost:3030/api/polls/${user.userId}`);
        const polls = await pollsResponse.json();

        return (
            <ClientProfile user={user} polls={polls} />
        );
    } catch {
        return redirect("/");
    }
}

function ClientProfile({ user, polls }: { user: any, polls: any[] }) {
    return (
        <div>
            <h1>Profile Page</h1>
            <p>
                User-ID: {user.userId}<br />
                Name: {user.fullName}
            </p>
            <ClientSideActions polls={polls} />
        </div>
    );
}
