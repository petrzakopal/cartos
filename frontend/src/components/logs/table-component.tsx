import { LogEntry, columns } from "@/components/logs/columns";
import { DataTable } from "@/components/logs/data-table";

// get logs

export const LogTable = ({loaderData}:{loaderData: LogEntry[]}) => {
    return (
        <>
            <div className="w-full relative">
                <DataTable columns={columns} data={loaderData} />
            </div>
        </>
    )
}
