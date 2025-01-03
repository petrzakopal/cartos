import { PageTitle } from "@/components/navigation";

import { useLocation } from 'react-router-dom'


type LoaderData = {
    users: [];
    title: string;
};

export async function clientLoader() {

    const req_body: string = JSON.stringify({})

    try {

        const response = await fetch(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/user/view/all`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();

        return {
            title: "Users",
            users: data
        };
    }
    catch (e) {
        console.error(e)
        return {
            title: "Users",
            users: JSON.parse('{"msg":"no users"}')
        };
    };
}

export default function Component({ loaderData }: { loaderData: LoaderData }) {
    const location = useLocation()
    let usrData;
    if (location.state && location.state.usrData != null) {
        usrData = location.state.usrData
    } else {
        usrData = null // or set to a default value
    }
    return (
        <>
            <div className="w-full flex flex-col">
                <PageTitle text={loaderData.title} />
                <pre className="whitespace-pre">{JSON.stringify(usrData, null, "\t")}</pre>
            </div>
        </>
    );
}
