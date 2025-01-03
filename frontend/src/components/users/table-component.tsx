import { UserEntry, columns } from "@/components/users/columns";
import { DataTable } from "@/components/users/data-table";

export const UserTable = ({loaderData}:{loaderData: UserEntry[]}) => {
    return (
        <>
            <div className="w-full relative">
                <DataTable columns={columns} data={loaderData} />
            </div>
        </>
    )
}
