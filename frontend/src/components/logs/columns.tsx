import { ColumnDef } from "@tanstack/react-table"
import { UserStatusValues } from "../users/editForm/formComponents"

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type LogEntry = {
  id: number
  timestamp: string
  card_serial_number: string
  status: UserStatusValues
  email: string
  note: string
}

export const columns: ColumnDef<LogEntry>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "timestamp",
    header: "Timestamp",
  },
  {
    accessorKey: "card_serial_number",
    header: "Card Serial Number",
  },
  {
    accessorKey: "status",
    header: "Status",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "note",
    header: "Note",
  }
]
