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
    const [selectedOption, setSelectedOption] = useState<Record<string, number | null>>({});

    const voteOnPoll = async (pollId: string) => {
        const selected = selectedOption[pollId];

        if (selected !== null) {
            const response = await fetch(`http://localhost:3030/api/votes`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    poll_id: pollId,
                    option_index: selected,
                }),
            });

            if (response.ok) {
                alert("Vote submitted!");
            } else {
                alert("Failed to submit vote.");
            }
        }
    };

    const handleOptionChange = (pollId: string, index: number) => {
        setSelectedOption((prev) => ({
            ...prev,
            [pollId]: index,
        }));
    };

    return (
        <div>
            <h2>Polls</h2>
            {polls.map((poll) => (
                <div key={`poll-${poll.id}`}>
                    <h3>{poll.question}</h3>
                    <div>
                        {poll.options.map((option, index) => (
                            <div key={`option-${poll.id}-${index}`}>
                                <label>
                                    <input
                                        type="radio"
                                        name={`poll-${poll.id}`}
                                        value={index}
                                        checked={selectedOption[poll.id] === index}
                                        onChange={() => handleOptionChange(poll.id, index)}
                                    />
                                    {option}
                                </label>
                            </div>
                        ))}
                    </div>
                    <button onClick={() => voteOnPoll(poll.id)}>Vote</button>
                </div>
            ))}
        </div>
    );
}
