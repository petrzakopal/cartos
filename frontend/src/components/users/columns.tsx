import { ColumnDef } from "@tanstack/react-table"
import { Link, NavLink } from "react-router"

// This type is used to define the shape of our data.
// You can use a Zod schema here if you want.
export type UserEntry = {
    id: number
    card_serial_number: string
    email: string
    note: string,
    status: string,
    updated_at: string
}

export const columns: ColumnDef<UserEntry>[] = [
    {
        accessorKey: "id",
        header: "ID",
    },
    {
        accessorKey: "status",
        header: "Status",
    },
    {
        accessorKey: "updated_at",
        header: "Updated at",
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
    },
    {
        accessorKey: "",
        header: "Action",
        cell: ({row }) => {
            const id = row.original.id
            return (
                <>
                    <div className="w-fit flex flex-row space-x-3 items-center">
                        <Link state={{usrData: row.original}} className={"w-fit px-2.5 py-1.5 rounded-md bg-tertiary text-white hover:underline"}
                            to={{
                            pathname: `/users/edit`,
                            search: `?id=${id}`,
                        }}>
                            Edit
                        </Link>

                        <span
                            onClick={async () => {
                                console.log("Performing deactivation of user", row.original)
                            }}
                            className={"hover:cursor-pointer w-fit px-2.5 py-1.5 rounded-md bg-secondary text-white hover:underline"}>Deactivate</span>
                    </div>
                </>
            )
        },
    },
]
