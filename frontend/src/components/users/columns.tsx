import { ColumnDef } from "@tanstack/react-table"
import { Link, NavigateFunction, NavLink } from "react-router"
import { updateUserEntry, UserEntry } from "./editForm/formComponents"
import { useState } from "react";


export const useColumns = (data: UserEntry[], setData : React.Dispatch<React.SetStateAction<UserEntry[]>>): ColumnDef<UserEntry>[] => {

    return [

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
            cell: ({ row }) => {
                const originalData: UserEntry = row.original;
                return (
                    <>
                        <div className="w-fit flex flex-row space-x-3 items-center">
                            <Link state={{ usrData: row.original }} className={"w-fit px-2.5 py-1.5 rounded-md bg-tertiary text-white hover:underline"}
                                to={{
                                    pathname: `/users/edit`,
                                }}>
                                Edit
                            </Link>

                            <span
                                onClick={async () => {

                                    const updatedData: UserEntry = { ...originalData, status: "deactivated" };

                                    const response = await updateUserEntry(updatedData);
                                    setData((prevData) =>
                                        prevData.map((entry) =>
                                            entry.id === originalData.id ? response.data : entry
                                        )
                                    );

                                    console.log(response)

                                    row.original = response.data
                                }}
                                className={"hover:cursor-pointer w-fit px-2.5 py-1.5 rounded-md bg-secondary text-white hover:underline"}>
                                Deactivate
                            </span>

                            <span
                                onClick={async () => {

                                    const updatedData: UserEntry = { ...originalData, status: "active" };

                                    const response = await updateUserEntry(updatedData);
                                    setData((prevData) =>
                                        prevData.map((entry) =>
                                            entry.id === originalData.id ? response.data : entry
                                        )
                                    );

                                    console.log(response)

                                    row.original = response.data
                                }}
                                className={"hover:cursor-pointer w-fit px-2.5 py-1.5 rounded-md bg-quaternary text-white hover:underline"}>
                                Activate
                            </span>
                        </div>
                    </>
                )
            },
        },


    ]
}


