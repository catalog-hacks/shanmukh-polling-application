"use client";

import { useEffect, useState } from "react";

interface PollResult {
  _id: string;
  count: number;
}

interface PollOption {
  id: string;
  option_text: string;
}

interface PollStatisticsProps {
  pollId: string;
  options: PollOption[];
}

export default function PollStatistics({ pollId, options }: PollStatisticsProps) {
  const [results, setResults] = useState<PollResult[]>([]);

  useEffect(() => {
    const fetchResults = async () => {
      try {
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/poll_results/${pollId}`
        );
        
        if (!response.ok) throw new Error("Failed to fetch poll results");
        
        const data = await response.json();
        
        if (!Array.isArray(data)) {
          console.error("Unexpected data format:", data);
          setResults([]);
          return;
        }

        setResults(data);
      } catch (error) {
        console.error("Error fetching poll results:", error);
        setResults([]);
      }
    };

    fetchResults();
  }, [pollId]);

  const totalVotes = results.length
    ? results.reduce((sum, result) => sum + (result.count || 0), 0)
    : 0;

  const getVoteCount = (optionId: string) =>
    results.find((result) => result._id === optionId)?.count || 0;

  const getPercentage = (optionId: string) => {
    const count = getVoteCount(optionId);
    return totalVotes ? ((count / totalVotes) * 100).toFixed(2) : "0";
  };

  return (
    <div className="p-4 bg-gray-100 rounded-md shadow-md">
      <ul className="space-y-2">
        {options.map((option) => (
          <li key={option.id} className="flex items-center">
            <span className="w-20">{option.option_text}</span>
            <div className="flex-grow mx-2 bg-gray-300 rounded">
              <div
                className="bg-blue-500 h-4 rounded"
                style={{ width: `${getPercentage(option.id)}%` }}
              ></div>
            </div>
            <span className="text-sm text-gray-600">
              {getVoteCount(option.id)} votes ({getPercentage(option.id)}%)
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}