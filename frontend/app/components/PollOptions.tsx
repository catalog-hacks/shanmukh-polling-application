"use client";
import React from "react";

interface PollOption {
  id: string;
  option_text: string;
}

interface PollOptionsProps {
  options: PollOption[];
  isMultipleChoice: boolean;
  selectedOptions: string[];
  handleOptionChange: (optionId: string) => void;
  isDisabled: boolean;
}

const PollOptions: React.FC<PollOptionsProps> = ({
  options,
  isMultipleChoice,
  selectedOptions,
  handleOptionChange,
  isDisabled,
}) => (
  <div className="space-y-2">
    {options.map((option) => (
      <div key={option.id} className="flex items-center">
        <input
          type={isMultipleChoice ? "checkbox" : "radio"}
          value={option.id}
          checked={selectedOptions.includes(option.id)}
          onChange={() => handleOptionChange(option.id)}
          disabled={isDisabled}
          className="mr-2"
        />
        <label>{option.option_text}</label>
      </div>
    ))}
  </div>
);

export default PollOptions;
