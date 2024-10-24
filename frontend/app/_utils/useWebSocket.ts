import { useState, useEffect, useCallback } from 'react';

export function useWebSocket(pollId: string) {
    const [ws, setWs] = useState<WebSocket | null>(null);
    const [results, setResults] = useState<Array<{ _id: string; count: number }>>([]);
    const [status, setStatus] = useState<{ poll_id: string, is_active: boolean } | null>(null);
    const [isReset, setIsReset] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const fetchInitialResults = useCallback(async () => {
        try {
            const response = await fetch(
                `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/poll_results/${pollId}`
            );
            if (!response.ok) throw new Error("Failed to fetch poll results");
            const data = await response.json();
            setResults(data);
        } catch (error) {
            console.error("Error fetching initial results:", error);
            setError("Failed to load initial results");
        }
    }, [pollId]);

    useEffect(() => {
        fetchInitialResults();

        const websocket = new WebSocket(
            `${process.env.NEXT_PUBLIC_BACKEND_URL?.replace('http', 'ws')}/ws/${pollId}`
        );

        websocket.onopen = () => {
            console.log('WebSocket Connected');
            setError(null);
        };

        websocket.onmessage = (event) => {
            try {
                const update = JSON.parse(event.data);
                if (update.VoteUpdate){
                    setResults(update.VoteUpdate.results);
                }

                if (update.Reset) {
                    setResults([]);
                    setIsReset(true);
                }

                if (update.StatusUpdate) {
                    setStatus(update.StatusUpdate);
                }
            } catch (error) {
                console.error('Error processing WebSocket message:', error);
            }
        };

        websocket.onerror = (error) => {
            console.error('WebSocket error:', error);
            setError('WebSocket connection error');
        };

        websocket.onclose = () => {
            console.log('WebSocket disconnected');
            setError('WebSocket connection closed');
        };

        setWs(websocket);

        return () => {
            websocket.close();
        };
    }, [pollId, fetchInitialResults]);

    return { results, status, isReset, error };
}