import { ColumnDef } from "@tanstack/react-table"

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type LogEntry = {
  id: number
  timestamp: string
  card_serial_number: string
  result: string
  status: "authenticated" | "not_authenticated"
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
    accessorKey: "result",
    header: "Result",
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
