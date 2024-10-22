"use client";
import React from "react";

interface PollActionsProps {
  isPollDisabled: boolean;
  hasChangedVote: boolean;
  previousVote: boolean;
  selectedOptions: string[];
  submitVote: () => void;
}

const PollActions: React.FC<PollActionsProps> = ({
  isPollDisabled,
  hasChangedVote,
  previousVote,
  selectedOptions,
  submitVote,
}) => (
  <button
    onClick={submitVote}
    disabled={
      isPollDisabled || selectedOptions.length === 0 || (!hasChangedVote && previousVote)
    }
    className={`mt-4 px-4 py-2 rounded-md ${
      isPollDisabled || selectedOptions.length === 0 || (!hasChangedVote && previousVote)
        ? "bg-gray-400 text-gray-700 cursor-not-allowed"
        : "bg-blue-500 text-white"
    }`}
  >
    {isPollDisabled
      ? "Poll is Disabled"
      : previousVote
      ? hasChangedVote
        ? "Change Vote"
        : "Already Voted"
      : "Submit Vote"}
  </button>
);

export default PollActions;
