import { LogEntry } from "@/components/logs/columns";
import { UserEntry } from "@/components/users/editForm/formComponents";
import { toast } from "sonner";

export const handleMessage = (message: WebsocketMessageBody) => {

    switch (message.action) {
        case MessageAction.CardRead:
            {
                toast.info("Read card: "+ message.data.card_serial_number + ".",
                    {
                        closeButton: true
                    })
                break;
            }
        case MessageAction.NewLogEntry:
            {

                toast.success("New log of user: " + message.data.email + ", with card_serial_number: " + message.data.card_serial_number + ".",
                    {
                        closeButton: true
                    })
                break;
            }
        case MessageAction.NewUserEntry:
            {
                break;
            }
    }
}

export enum MessageAction {
    CardRead = "CardRead",
    NewLogEntry = "NewLogEntry",
    NewUserEntry = "NewUserEntry"
}

export type WebsocketMessageBody = {
    action: MessageAction,
    data: LogEntry | UserEntry
}
