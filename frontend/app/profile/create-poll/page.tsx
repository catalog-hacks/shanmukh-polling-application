"use client";
import { useState } from "react";
import { FaTrashAlt } from "react-icons/fa";
import { useUser } from "../../_utils/UserContext";
import toast from "react-hot-toast";

export default function CreatePollPage() {
  const [question, setQuestion] = useState("");
  const [options, setOptions] = useState<string[]>(["", ""]);
  const [multipleChoices, setMultipleChoices] = useState(false);

  const { user } = useUser();
  const user_id = user?.userId;
  
  const addOption = () => setOptions([...options, ""]);

  const removeOption = (index: number) =>
    setOptions(options.filter((_, i) => i !== index));

  const handleChangeOption = (index: number, value: string) => {
    const newOptions = [...options];
    newOptions[index] = value;
    setOptions(newOptions);
  };

  const createPoll = async () => {
    try {
      const backend_url = process.env.NEXT_PUBLIC_BACKEND_URL;
      const filteredOptions = options.filter(option => option.trim() !== "");
      if (filteredOptions.length <= 1) {
        toast.error("Please provide at least two options.");
        return;
      }
      const response = await fetch(`${backend_url}/api/create_polls`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          created_by: user_id,
          question,
          options: filteredOptions,
          is_multiple_choice: multipleChoices,
        }),
      });

      if (response.ok) {
        alert("Poll created successfully!");
        setQuestion("");
        setOptions(["", ""]);
        setMultipleChoices(false);
      } else {
        alert("Failed to create poll.");
      }
    } catch (error) {
      console.error("Error creating poll:", error);
      alert("An error occurred.");
    }
  };

  return (
    <div className="max-w-lg mx-auto bg-white p-6 rounded-lg shadow-lg">
      <h2 className="text-2xl font-semibold mb-4">Create a New Poll</h2>
      <input
        type="text"
        placeholder="Enter your question"
        value={question}
        onChange={(e) => setQuestion(e.target.value)}
        className="w-full mb-4 p-3 border border-gray-300 rounded-md"
      />
      {options.map((option, index) => (
        <div key={index} className="flex items-center mb-3">
          <input
            type="text"
            placeholder={`Option ${index + 1}`}
            value={option}
            onChange={(e) => handleChangeOption(index, e.target.value)}
            className="flex-grow p-3 border border-gray-300 rounded-md"
          />
          {options.length > 2 && (
            <button
              onClick={() => removeOption(index)}
              className="ml-2 text-red-500"
            >
              <FaTrashAlt />
            </button>
          )}
        </div>
      ))}
      <div className="flex items-center mt-4">
        <input
          type="checkbox"
          checked={multipleChoices}
          onChange={(e) => setMultipleChoices(e.target.checked)}
          className="mr-2"
        />
        <label>Allow multiple choices</label>
      </div>
      <div className="flex space-x-4 mt-4">
        <button
          onClick={addOption}
          className="px-4 py-2 bg-green-500 text-white rounded-md"
        >
          Add Option
        </button>
        <button
          onClick={createPoll}
          className="px-4 py-2 bg-blue-500 text-white rounded-md"
        >
          Create Poll
        </button>
      </div>
    </div>
  );
}
