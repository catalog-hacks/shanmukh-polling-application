"use client";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useUser } from "../../../_utils/UserContext";
import PollOptions from "../../../components/PollOptions";
import PollQuestion from "../../../components/PollQuestion";
import PollActions from "../../../components/PollActions";
import PollStatistics from "../../../components/PollStatistics";
import Spinner from "@/app/_utils/Spinner";
import PollResultGraph from "@/app/components/PollResultGraph";
import toast from "react-hot-toast";
import { useWebSocket } from "@/app/_utils/useWebSocket";

interface PollOption {
  id: string;
  option_text: string;
}

interface Poll {
  id: string;
  question: string;
  options: PollOption[];
  is_multiple_choice: boolean;
  isactive: boolean;
}

interface Vote {
  option_ids: { $oid: string }[];
}

export default function PollDetailPage({
  params,
}: {
  params: { poll_id: string };
}) {
  const [poll, setPoll] = useState<Poll | null>(null);
  const [selectedOptions, setSelectedOptions] = useState<string[]>([]);
  const [previousVote, setPreviousVote] = useState<Vote | null>(null);
  const { user } = useUser();
  const router = useRouter();

  // Use WebSocket to receive real-time poll updates
  const { results, status, isReset, error } = useWebSocket(params.poll_id);

  useEffect(() => {
    const fetchPollAndVote = async () => {
      try {
        const token = localStorage.getItem("token");
        if (!token) {
          toast.error("Please log in.");
          router.push("/");
          return;
        }

        const pollResponse = await fetch(
          `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/polls/${params.poll_id}`,
          { headers: { Authorization: `Bearer ${token}` } }
        );

        if (!pollResponse.ok) throw new Error("Failed to fetch poll");

        const pollData = await pollResponse.json();

        const formattedOptions: PollOption[] = pollData.options.map(
          (option: [any, string]) => ({
            id: option[0].$oid,
            option_text: option[1],
          })
        );

        setPoll({
          id: pollData._id.$oid,
          question: pollData.question,
          options: formattedOptions,
          is_multiple_choice: pollData.is_multiple_choice,
          isactive: pollData.isactive,
        });

        const voteResponse = await fetch(
          `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/votes/${params.poll_id}/${user?.userId}`,
          { headers: { Authorization: `Bearer ${token}` } }
        );

        if (voteResponse.ok) {
          const voteData: Vote = await voteResponse.json();
          const selectedIds = voteData.option_ids.map((option) => option.$oid);

          setPreviousVote(voteData);
          setSelectedOptions(selectedIds);
        }
      } catch (error) {
        console.error("Error fetching poll or vote:", error);
        toast.error("An error occurred while fetching poll or vote.");
      }
    };

    if (user) fetchPollAndVote();
  }, [params.poll_id, user]);

  useEffect(() => {
    // Update poll state in real-time when WebSocket sends new results
    if (results.length > 0 && poll) {
      setPoll((prevPoll) => ({
        ...prevPoll!,
        options: prevPoll!.options.map((option) => ({
          ...option,
          votes: results.find((res) => res._id === option.id)?.count || 0,
        })),
      }));
    }
  }, [results]);

  useEffect(() => {
    // Handle poll reset event from WebSocket
    if (status?.poll_id === poll?.id) {
      if (status?.is_active === false && poll?.isactive) {
        setPoll((prevPoll) => ({
          ...prevPoll!,
          isactive: false,
        }));
        toast.error("This Poll has been disabled");
      } else if (status?.is_active === true && !poll?.isactive) {
        setPoll((prevPoll) => ({
          ...prevPoll!,
          isactive: true,
        }));
        toast.success("This Poll has been enabled");
      }
    }
  }, [status, poll]);

  useEffect(() => {
    // Update poll state in real-time when WebSocket sends new results
    if (results.length > 0 && poll) {
      setPoll((prevPoll) => ({
        ...prevPoll!,
        options: prevPoll!.options.map((option) => ({
          ...option,
          votes: results.find((res) => res._id === option.id)?.count || 0,
        })),
      }));
    }
  }, [results]);
  useEffect(() => {
    // Reset poll state every time the WebSocket signals a reset
    if (isReset && poll) {
      handlePollReset();
    }
  }, [isReset]);
  const handlePollReset = () => {
    // Clear selected options and previous votes
    setSelectedOptions([]);
    setPreviousVote(null);
    setPoll((prevPoll) => ({
      ...prevPoll!,
      options: prevPoll!.options.map((option) => ({ ...option, votes: 0 })),
    }));

    toast.success("Poll has been reset. Please vote again.");
  };

  const handleOptionChange = (optionId: string) => {
    if (poll?.is_multiple_choice) {
      setSelectedOptions((prev) =>
        prev.includes(optionId)
          ? prev.filter((id) => id !== optionId)
          : [...prev, optionId]
      );
    } else {
      setSelectedOptions([optionId]);
    }
  };

  const hasChangedVote = () =>
    JSON.stringify(selectedOptions.sort()) !==
    JSON.stringify(previousVote?.option_ids.map((opt) => opt.$oid).sort());

  const submitVote = async () => {
    if (!user) return toast.error("Please log in to vote.");
    if (selectedOptions.length === 0)
      return toast.error("Select at least one option.");

    try {
      const token = localStorage.getItem("token");
      if (!token) {
        toast.error("No token found. Please log in again.");
        router.push("/");
        return;
      }

      const response = await fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/vote`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({
            poll_id: params.poll_id,
            option_ids: selectedOptions,
          }),
        }
      );

      if (response.ok) {
        toast.success(previousVote ? "Vote updated!" : "Vote submitted!");
        setPreviousVote({
          option_ids: selectedOptions.map((id) => ({ $oid: id })),
        });
      } else {
        const errorData = await response.json();
        toast.error(errorData.message || "Failed to submit vote.");
      }
    } catch (error) {
      console.error("Error submitting vote:", error);
      toast.error("An error occurred while submitting your vote.");
    }
  };

  if (!poll) return <Spinner />;

  return (
    <div className="flex justify-center p-6">
      <div className="flex flex-col space-y-8 w-full max-w-6xl">
        <div className="flex flex-col md:flex-row space-y-8 md:space-y-0 md:space-x-8">
          <div className="bg-white p-6 rounded-lg shadow-lg w-full md:w-1/2">
            <PollQuestion question={poll.question} isactive={poll.isactive} />
            <div className="mt-4">
              <PollOptions
                options={poll.options}
                isMultipleChoice={poll.is_multiple_choice}
                selectedOptions={selectedOptions}
                handleOptionChange={handleOptionChange}
                isDisabled={!poll.isactive}
              />
            </div>
            <div className="mt-6">
              <PollActions
                isPollDisabled={!poll.isactive}
                hasChangedVote={hasChangedVote()}
                previousVote={previousVote !== null}
                selectedOptions={selectedOptions}
                submitVote={submitVote}
              />
            </div>
          </div>

          <div className="bg-gray-50 p-6 rounded-lg shadow-lg w-full md:w-1/2">
            <h2 className="text-xl font-semibold mb-4 text-center">Results</h2>
            <PollStatistics pollId={poll.id} options={poll.options} />
          </div>
        </div>

        <div className="flex justify-center">
          <div className="bg-gray-50 p-6 rounded-lg shadow-lg w-1/2">
            <h2 className="text-xl font-semibold mb-4 text-center">
              Result Graphs
            </h2>
            <PollResultGraph pollId={poll.id} options={poll.options} />
          </div>
        </div>
      </div>
    </div>
  );
}
