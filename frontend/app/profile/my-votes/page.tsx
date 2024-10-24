"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Spinner from "@/app/_utils/Spinner";
import toast from "react-hot-toast";

interface Poll {
  _id: { $oid: string };
  question: string;
  isactive: boolean;
}

export default function MyVotesPage() {
  const [votedPolls, setVotedPolls] = useState<Poll[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    const fetchVotedPolls = async () => {
      try {
        setLoading(true);
        const token = localStorage.getItem("token");

        if (!token) {
          toast.error("Please log in to view your votes.");
          router.push("/");
          return;
        }

        const backend_url = process.env.NEXT_PUBLIC_BACKEND_URL;
        const response = await fetch(`${backend_url}/api/my_votes`, {
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
        });

        if (!response.ok) {
          throw new Error("Failed to fetch voted polls.");
        }

        const data = await response.json();
        setVotedPolls(data);
      } catch (error) {
        console.error("Error fetching voted polls:", error);
        setError("Unable to load your votes. Please try again later.");
        toast.error("Unable to load your votes.");
      } finally {
        setLoading(false);
      }
    };

    fetchVotedPolls();
  }, [router]);

  if (loading) return <Spinner />;
  if (error) return <p className="p-6 text-center text-red-500">{error}</p>;
  if (votedPolls.length === 0) 
    return <p className="p-6 text-center">No voted polls available.</p>;

  const getStatusColor = (isActive: boolean) =>
    isActive ? "text-green-500" : "text-red-500";

  return (
    <div className="p-6">
      <h2 className="text-2xl font-semibold mb-4">My Votes</h2>
      <ul className="space-y-4">
        {votedPolls.map((poll) => (
          <li
            key={poll._id.$oid}
            className="p-4 bg-white rounded-md shadow-md cursor-pointer hover:bg-gray-100"
            onClick={() => router.push(`/profile/all-polls/${poll._id.$oid}`)}
          >
            <h3 className="text-xl font-semibold">{poll.question}</h3>
            <p
              className={`text-sm pt-5 font-semibold ${getStatusColor(
                poll.isactive
              )}`}
            >
              Status: {poll.isactive ? "Active" : "Inactive"}
            </p>
          </li>
        ))}
      </ul>
    </div>
  );
}
