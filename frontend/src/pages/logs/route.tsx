import { LogTable } from "@/components/logs/table-component";
import { PageTitle } from "@/components/navigation";
import { getApiUrl } from "@/lib/useApiUrl";
import { Helmet } from "react-helmet";

type LoaderData = {
  logs: [];
  title: string;
};

export async function clientLoader() {

    const req_body: string = JSON.stringify({})

    try {

        const response = await fetch(`${getApiUrl()}/log/view/all`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();

        return {
            title: "Logs",
            logs: data
        };
    }
    catch (e) {
        console.error(e)
        return {
            title: "Logs",
            logs: JSON.parse('{"msg":"no logs"}')
        };
    };
}

export default function Component({ loaderData }:{loaderData: LoaderData}) {
    return (
        <>
            <Helmet>
                <title>Logs</title>
            </Helmet>
            <div className="w-full flex flex-col">
                <PageTitle text={loaderData.title}/>
                <LogTable loaderData={loaderData.logs} />
            </div>
        </>
    );
}
