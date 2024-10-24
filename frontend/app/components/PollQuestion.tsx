"use client";
import React from "react";

interface PollQuestionProps {
  question: string;
  isactive: boolean;
}

const PollQuestion: React.FC<PollQuestionProps> = ({ question, isactive }) => (
  <div className={`mb-4 ${!isactive ? "opacity-50" : ""}`}>
    <h2 className="text-2xl font-semibold">{question}</h2>
    <p className={`text-sm pt-2 font-semibold ${isactive ? "text-green-500" : "text-red-500"}`}>
      Status: {isactive ? "Active" : "Inactive"}
    </p>
  </div>
);

export default PollQuestion;