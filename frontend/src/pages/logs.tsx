import { NavLink } from "react-router";


export async function clientLoader() {

    const req_body: string = JSON.stringify({})

    try {

        const response = await fetch(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/log/view/all`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();

        return {
            title: "Logs page",
            logs: data
        };
    }
    catch (e) {
        console.error(e)
        return {
            title: "Logs page",
            logs: JSON.parse('{"msg":"no logs"}')
        };
    };
}


export default function Component({ loaderData }) {
    return (
        <>
            <div className="w-full flex flex-col">
                <h1>{loaderData.title}</h1>
                <NavLink to={"/"} >To Home</NavLink>
                <pre className="whitespace-pre-wrap text-black">{JSON.stringify(loaderData.logs, null, "\t")}</pre>
            </div>
        </>
    );
}
