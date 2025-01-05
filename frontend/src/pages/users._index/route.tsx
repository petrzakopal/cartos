import { PageTitle } from "@/components/navigation";
import { UserTable } from "@/components/users/table-component";
import { NavLink } from "react-router";

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
    return (
        <>
            <div className="w-full flex flex-col">
                <PageTitle text={loaderData.title} />
                <div className="w-full flex flex-row relative">
                    <NavLink to={"new"} className={"ml-auto hover:underline w-fit"}>Add new user</NavLink>
                </div>
                <UserTable loaderData={loaderData.users} />
            </div>
        </>
    );
}
