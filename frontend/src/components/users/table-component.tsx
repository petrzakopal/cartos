import { useColumns } from "@/components/users/columns";
import { DataTable } from "@/components/users/data-table";
import { useState } from "react";
import { UserEntry } from "@/components/users/editForm/formComponents";


export const UserTable = ({ loaderData }: { loaderData: UserEntry[] }) => {

    const [data, setData] = useState<UserEntry[]>(loaderData);
    return (
        <>
            <div className="w-full relative">
                <DataTable columns={useColumns(data, setData)} data={data} />
            </div>
        </>
    )
}
