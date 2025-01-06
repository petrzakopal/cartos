
import { ResetButton } from "@/components/hw/system_restart";
import { PageTitle } from "@/components/navigation";
import { SpaceY } from "@/components/space";
import { getApiUrl } from "@/lib/useApiUrl";
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
                <PageTitle text={loaderData.title} />
                <SpaceY mt="mt-12" />
                <pre className="whitespace-pre-wrap">
                    backend url: {getApiUrl()}
                </pre>
                <span>Source code of the application is Open Source at <NavLink className="italic text-primary" to={"https://github.com/petrzakopal/cartos"}>petrzakopal/cartos</NavLink>.</span>
                <p>If there are any problems with the application contact the maintainers of the repository.</p>
                <SpaceY mt="mt-12" />
                <span className="text-xl font-bold">Control panel</span>
                <SpaceY mt="mt-4" />
                <ResetButton />
            </div>
        </>
    );
}
