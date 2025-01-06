
import { ResetButton } from "@/components/hw/system_restart";
import { SpaceY } from "@/components/space";
import { NavLink } from "react-router";


export async function clientLoader() {
    // you can now fetch data here
    return {
        title: "About page",
    };
}

// Define loader function only if SSR is disabled
//export async function loader({ params }: Route.LoaderArgs) {
//        console.log("server side variable BACKEND_URL:", process.env.BACKEND_URL);
//}

export default function Component({ loaderData }) {
    return (
        <>
            <div className="w-full flex flex-col">
                <h1>{loaderData.title}</h1>
                <span className="italic text-primary">under development</span>
                <SpaceY mt="mt-5" />
                <pre className="whitespace-pre-wrap">
                    backend url: {import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}
                </pre>
                <ResetButton />
            </div>
        </>
    );
}
