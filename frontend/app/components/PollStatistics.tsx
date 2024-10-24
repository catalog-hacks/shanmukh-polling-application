"use client";
import { useWebSocket } from "../_utils/useWebSocket";

interface PollOption {
  id: string;
  option_text: string;
}

interface PollStatisticsProps {
  pollId: string;
  options?: PollOption[];
}

export default function PollStatistics({
  pollId,
  options,
}: PollStatisticsProps) {
  const { results, error } = useWebSocket(pollId);

  const totalVotes = results.reduce(
    (sum, result) => sum + (result.count || 0),
    0
  );

  const getVoteCount = (optionId: string) =>
    results.find((result) => result._id === optionId)?.count || 0;

  const getPercentage = (optionId: string) => {
    const count = getVoteCount(optionId);
    return totalVotes ? ((count / totalVotes) * 100).toFixed(2) : "0";
  };
  return (
    <div className="bg-white rounded-lg shadow p-6">
      <ul className="space-y-4">
        {options?.map((option) => (
          <li key={option.id} className="flex items-center justify-between">
            <div className="flex-1">
              {option.option_text}
              <div className="w-full bg-gray-200 rounded-full h-2.5 mb-4">
                <div
                  className="bg-blue-600 h-2.5 rounded-full"
                  style={{ width: `${getPercentage(option.id)}%` }}
                />
              </div>
            </div>
            <div className="ml-4 text-sm text-gray-600">
              {getVoteCount(option.id)} votes ({getPercentage(option.id)}%)
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
