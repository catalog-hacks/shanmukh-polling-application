"use client";

import { useEffect, useState, useCallback } from "react";
import { useUser } from "../../_utils/UserContext";
import { useRouter } from "next/navigation";
import toast from "react-hot-toast";

interface Poll {
  _id: { $oid: string };
  question: string;
  created_at: string;
  isactive: boolean;
  created_by: string;
}

export default function MyPollsPage() {
  const [polls, setPolls] = useState<Poll[]>([]);
  const { user } = useUser();
  const router = useRouter();

  const fetchPolls = useCallback(async () => {
    if (!user) return;

    try {
      const token = localStorage.getItem("token");
      if (!token) {
        toast.error("Please log in again.");
        router.push("/");
        return;
      }

      const response = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/polls/user/${user.userId}`,
        {
          headers: { Authorization: `Bearer ${token}` },
        }
      );

      if (!response.ok) throw new Error("Failed to fetch polls.");

      const data = await response.json();
      setPolls(data);
    } catch (error) {
      console.error("Error fetching polls:", error);
      toast.error("Failed to load polls. Please try again.");
    }
  }, [user, router]);

  useEffect(() => {
    fetchPolls();
  }, [fetchPolls]);

  const formatDate = (dateString: string) =>
    new Date(dateString).toLocaleDateString("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    });

  const resetVotes = async (pollId: string) => {
    try {
      const token = localStorage.getItem("token");
      if (!token) {
        toast.error("Please log in again.");
        router.push("/");
        return;
      }

      const response = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/reset_votes/${pollId}`,
        {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
        }
      );

      if (response.ok) {
        toast.success("Votes reset successfully.");
        fetchPolls();
      } else {
        const data = await response.json();
        toast.error(data.message || "Failed to reset votes.");
      }
    } catch (error) {
      console.error("Error resetting votes:", error);
      toast.error("An error occurred. Please try again.");
    }
  };

  const togglePollStatus = async (pollId: string, newStatus: boolean) => {
    try {
      const token = localStorage.getItem("token");
      if (!token) {
        toast.error("Please log in again.");
        router.push("/");
        return;
      }

      const response = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/toggle_poll_status/${pollId}`,
        {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ isactive: newStatus }),
        }
      );

      if (response.ok) {
        toast.success(
          `Poll ${newStatus ? "enabled" : "disabled"} successfully.`
        );
        fetchPolls();
      } else {
        const data = await response.json();
        toast.error(data.message || "Failed to update poll status.");
      }
    } catch (error) {
      console.error("Error updating poll status:", error);
      toast.error("An error occurred. Please try again.");
    }
  };

  const getStatusColor = (isActive: boolean) =>
    isActive ? "text-green-500" : "text-red-500";

  if (!polls.length) {
    return <p className="text-center mt-6">No polls available.</p>;
  }

  return (
    <div className="p-6">
      <h2 className="text-2xl font-semibold mb-4">My Polls</h2>
      <ul className="space-y-4">
        {polls.map((poll) => (
          <li
            key={poll._id.$oid}
            className="p-4 bg-white rounded-md shadow-md cursor-pointer hover:bg-gray-100"
            onClick={() => router.push(`/profile/all-polls/${poll._id.$oid}`)}
          >
            <h3 className="text-xl font-semibold">{poll.question}</h3>
            <p className="text-sm text-gray-500">
              Created on: {formatDate(poll.created_at)}
            </p>
            <p
              className={`text-sm pt-5 font-semibold ${getStatusColor(
                poll.isactive
              )}`}
            >
              Status: {poll.isactive ? "Active" : "Inactive"}
            </p>

            <button
              onClick={(e) => {
                e.stopPropagation();
                resetVotes(poll._id.$oid);
              }}
              className="mt-4 mr-2 px-4 py-2 bg-red-500 text-white rounded-md"
            >
              Reset Votes
            </button>

            <button
              onClick={(e) => {
                e.stopPropagation();
                togglePollStatus(poll._id.$oid, !poll.isactive);
              }}
              className={`mt-4 px-4 py-2 rounded-md text-white ${
                poll.isactive ? "bg-blue-600" : "bg-green-500"
              }`}
            >
              {poll.isactive ? "Disable Poll" : "Enable Poll"}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
