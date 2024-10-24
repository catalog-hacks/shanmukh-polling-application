"use client";
import React from "react";

interface OptionWithVotes {
  id: string;
  option_text: string;
  votes: number;
  totalVotes: number;
}

interface PollResultsProps {
  results: OptionWithVotes[];
}

const PollResults: React.FC<PollResultsProps> = ({ results }) => (
  <div className="mt-6">
    <h3 className="text-xl font-semibold mb-2">Results</h3>
    {results.map((option) => (
      <div key={option.id} className="mb-2">
        <span className="block text-sm">{option.option_text}</span>
        <div className="relative w-full h-4 bg-gray-200 rounded-md">
          <div
            className="absolute top-0 left-0 h-full bg-blue-500 rounded-md"
            style={{ width: `${(option.votes / option.totalVotes) * 100}%` }}
          ></div>
        </div>
        <span className="text-xs text-gray-500">
          {option.votes} votes ({Math.round((option.votes / option.totalVotes) * 100)}%)
        </span>
      </div>
    ))}
  </div>
);

export default PollResults;