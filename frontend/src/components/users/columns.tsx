import { ColumnDef } from "@tanstack/react-table"

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type UserEntry = {
  id: number
  card_serial_number: string
  email: string
  note: string
}

export const columns: ColumnDef<UserEntry>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "card_serial_number",
    header: "Card Serial Number",
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
