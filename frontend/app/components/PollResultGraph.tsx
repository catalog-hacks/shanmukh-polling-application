"use client";

import { useState, useEffect } from "react";
import { Pie, Bar } from "react-chartjs-2";
import {
  Chart as ChartJS,
  ArcElement,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
} from "chart.js";
import { useWebSocket } from "../_utils/useWebSocket";

ChartJS.register(
  ArcElement,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement
);

interface PollResultGraphProps {
  options: { id: string; option_text: string }[];
  pollId: string;
}

interface VoteResult {
  _id: string;
  count: number;
}

const PollResultGraph: React.FC<PollResultGraphProps> = ({ options, pollId }) => {
  const [graphType, setGraphType] = useState<"pie" | "bar">("pie");
  const { results: votes, error } = useWebSocket(pollId);
  const safeVotes = votes || [];

  const data = {
    labels: options.map((option) => option.option_text),
    datasets: [
      {
        label: "Votes",
        data: options.map(
          (option) => safeVotes.find((vote) => vote._id === option.id)?.count || 0
        ),
        backgroundColor: ["#3498db", "#2ecc71", "#e74c3c", "#f1c40f"],
        borderWidth: 1,
      },
    ],
  };

  const renderGraph = () => {
    const chartStyle = { width: "50%", margin: "0 auto" };

    switch (graphType) {
      case "bar":
        return <Bar data={data} />;
      case "pie":
      default:
        return (
          <div style={chartStyle}>
            <Pie data={data} />
          </div>
        );
    }
  };

  return (
    <div className="mt-8">
      <div className="flex items-center mb-4">
        <label className="mr-2">Graph Type:</label>
        <select
          value={graphType}
          onChange={(e) => setGraphType(e.target.value as "pie" | "bar")}
          className="border rounded px-2 py-1"
        >
          <option value="pie">Pie Chart</option>
          <option value="bar">Bar Chart</option>
        </select>
      </div>
      {renderGraph()}
    </div>
  );
};

export default PollResultGraph;
