import { getApiUrl } from "@/lib/useApiUrl";
import { handleMessage } from "@/lib/websockets";
import { useEffect, useState } from "react";

export const WebsocketDisplayer = () => {

    const [message, setMessage] = useState("{}");
    const [ws, setWs] = useState<WebSocket>(null);

    useEffect(() => {
        const socket = new WebSocket(`${getApiUrl()}/ws`);

        socket.onopen = () => {
            console.log("Websocket connection estabilished.")
        }

        socket.onmessage = (event) => {
            console.log("Message from server:", event.data);
            setMessage(event.data);

            const parsed_message = JSON.parse(event.data);

            handleMessage(parsed_message);

            console.log("parsed message: ", JSON.parse(event.data))
        }

        socket.onerror = (error) => {
            console.error("Websocket error:", error);
        }

        socket.onclose = () => {
            console.warn("Websocket connection closed.")
        }

        setWs(socket);


        return () => {
            if (socket) {
                socket.close()
            }
        }
    }, [])

    return(
    <>
        </>
    )

}
