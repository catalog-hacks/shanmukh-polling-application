"use client";

import { useState } from "react";

interface Poll {
    id: string;
    question: string;
    options: string[];
}

interface PollListProps {
    polls: Poll[];
}

export default function PollList({ polls }: PollListProps) {
    const [selectedOption, setSelectedOption] = useState<number | null>(null);

    const voteOnPoll = async (pollId: string) => {
        if (selectedOption !== null) {
            const response = await fetch(`http://localhost:3030/api/votes`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    poll_id: pollId,
                    option_index: selectedOption,
                }),
            });

            if (response.ok) {
                alert("Vote submitted!");
            } else {
                alert("Failed to submit vote.");
            }
        }
    };

    return (
        <div>
            <h2>Polls</h2>
            {polls.map((poll) => (
                <div key={poll.id}>
                    <h3>{poll.question}</h3>
                    {poll.options.map((option: string, index: number) => (
                        <label key={index}>
                            <input
                                type="radio"
                                value={index}
                                checked={selectedOption === index}
                                onChange={() => setSelectedOption(index)}
                            />
                            {option}
                        </label>
                    ))}
                    <button onClick={() => voteOnPoll(poll.id)}>Vote</button>
                </div>
            ))}
        </div>
    );
}
