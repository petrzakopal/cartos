import { LogTable } from "@/components/logs/table-component";
import { PageTitle } from "@/components/navigation";
import { Helmet } from "react-helmet";
import { useEffect, useState } from "react";

type LoaderData = {
  logs: [];
  title: string;
};

export async function clientLoader() {

    const req_body: string = JSON.stringify({})

    try {

        const response = await fetch(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/log/view/all`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();

        return {
            title: "Logs with websockets",
            logs: data
        };
    }
    catch (e) {
        console.error(e)
        return {
            title: "Logs with websockets",
            logs: JSON.parse('{"msg":"no logs"}')
        };
    };
}


enum MessageAction {
    CardRead,
    NewLogEntry,
    NewUserEntry
}

export default function Component({ loaderData }:{loaderData: LoaderData}) {

    const [message, setMessage] = useState("");
    const [ws, setWs] = useState<WebSocket>(null);

    useEffect(() => {
        const socket = new WebSocket(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/ws`);

        socket.onopen = () => {
            console.log("Websocket connection estabilished.")
        }

        socket.onmessage = (event) => {
            console.log("Message from server:", event.data);
            console.log("parsed message: ", JSON.parse(message))
            setMessage(event.data);
        }

        socket.onerror = (error) => {
            console.error("Websocket error:", error);
        }

        socket.onclose = () => {
            console.warn("Websocket connection closed.")
        }

        setWs(socket);


        return () => {
            if(socket) {
                socket.close()
            }
        }
    }, [])

    const message_body = {
        action: MessageAction.CardRead,
        data: {
            id: 0,
            timestamp: "2025-01-03 12:56:30",
            card_serial_number: "04:4C:21:6A:2C:59:81",
            email: "",
            result: "not_authenticated",
            note: null
        }
    }

    const message_body_serialized = JSON.stringify(message_body);

    const sendMessage = () => {
        if(ws) {
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
                <PageTitle text={loaderData.title}/>
                <span>message from server</span>
                <pre className="whitespace-pre">{message}</pre>
                <button className="w-fit p-3 text-white bg-tertiary" onClick={sendMessage}>Send message</button>

                {/*<LogTable loaderData={loaderData.logs} />*/}
            </div>
        </>
    );
}
