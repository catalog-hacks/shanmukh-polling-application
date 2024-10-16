"use client";

import { useState } from "react";

export default function CreatePollForm() {
    const [question, setQuestion] = useState("");
    const [options, setOptions] = useState<string[]>(["", ""]);

    const addOption = () => {
        setOptions([...options, ""]);
    };

    const handleChangeOption = (index: number, value: string) => {
        const newOptions = [...options];
        newOptions[index] = value;
        setOptions(newOptions);
    };

    const createPoll = async () => {
        const response = await fetch("http://localhost:3030/api/polls", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                question,
                options,
            }),
        });

        if (response.ok) {
            alert("Poll created successfully!");
        } else {
            alert("Failed to create poll.");
        }
    };

    return (
        <div>
            <h2>Create a new poll</h2>
            <input
                type="text"
                placeholder="Question"
                value={question}
                onChange={(e) => setQuestion(e.target.value)}
            />
            {options.map((option, index) => (
                <input
                    key={index}
                    type="text"
                    placeholder={`Option ${index + 1}`}
                    value={option}
                    onChange={(e) => handleChangeOption(index, e.target.value)}
                />
            ))}
            <button onClick={addOption}>Add another option</button>
            <button onClick={createPoll}>Create Poll</button>
        </div>
    );
}
