"use client";

import LogoutButton from "@/app/_utils/LogoutButton";
import PollList from "./PollList";
import CreatePollForm from "./CreatePollForm";

export default function ClientSideActions({ polls }: { polls: any[] }) {
    return (
        <div>
            <LogoutButton />
            <PollList polls={polls} />
            <CreatePollForm />
        </div>
    );
}
