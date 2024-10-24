"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

interface Poll {
  id: string;
  question: string;
  created_by: string;
  created_at: string;
  isactive: boolean;
}

export default function AllPollsPage() {
  const [polls, setPolls] = useState<Poll[]>([]);
  const router = useRouter();

  useEffect(() => {
    const fetchPolls = async () => {
      const response = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/all_polls_summary`);
      const data = await response.json();
      setPolls(data);
    };

    fetchPolls();
  }, []);

  const formatDate = (dateString: string) =>
    new Date(dateString).toLocaleDateString("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    });

  const getStatusColor = (isActive: boolean) =>
    isActive ? "text-green-500" : "text-red-500";

  return (
    <div className="p-6">
      <h2 className="text-2xl font-semibold mb-4">All Polls</h2>
      <ul className="space-y-4">
        {polls.map((poll) => (
          <li
            key={poll.id}
            className="p-4 bg-white rounded-md shadow-md cursor-pointer hover:bg-gray-100"
            onClick={() => router.push(`/profile/all-polls/${poll.id}`)}
          >
            <h3 className="text-xl font-semibold">{poll.question}</h3>
            <p className="text-sm pt-2 text-gray-500">Created by: {poll.created_by}</p>
            <p className="text-sm text-gray-500">Created on: {formatDate(poll.created_at)}</p>
            <p className={`text-sm pt-5 font-semibold ${getStatusColor(poll.isactive)}`}>
              Status: {poll.isactive ? "Active" : "Inactive"}
            </p>
          </li>
        ))}
      </ul>
    </div>
  );
}