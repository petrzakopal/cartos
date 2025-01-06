import { PageTitle } from "@/components/navigation";
import { Helmet } from "react-helmet";
import { useEffect, useState } from "react";
import { LogEntry } from "@/components/logs/columns";
import { UserEntry } from "@/components/users/editForm/formComponents";
import { toast } from "sonner";
import { getApiUrl } from "@/lib/useApiUrl";
import { handleMessage, MessageAction, WebsocketMessageBody } from "@/lib/websockets";

type LoaderData = {
    logs: [];
    title: string;
};

export async function clientLoader() {

        return {
            title: "Logs with websockets",
        };
}



export default function Component({ loaderData }: { loaderData: LoaderData }) {

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

    const message_body: WebsocketMessageBody = {
        action: MessageAction.CardRead,
        data: {
            id: 0,
            timestamp: "2025-01-03 12:56:30",
            card_serial_number: "04:4C:21:6A:2C:59:81",
            email: "test@example.com",
            status: "deactivated",
            note: "",
        }
    }

    const message_body_serialized = JSON.stringify(message_body);

    const sendMessage = () => {
        if (ws) {
            //ws.send("Hello from the react vite frontend.")
            ws.send(message_body_serialized)
        }
    }

    return (
        <>
            <Helmet>
                <title>Logs</title>
            </Helmet>
            <div className="w-full flex flex-col">
                <PageTitle text={loaderData.title} />
                <span>message from server</span>
                <pre className="whitespace-pre">{message}</pre>
                <button className="w-fit p-3 text-white bg-tertiary" onClick={sendMessage}>Send test message</button>
            </div>
        </>
    );
}

