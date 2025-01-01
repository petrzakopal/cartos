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
                <span>test text about</span>
                <NavLink to={"/"} >To Home</NavLink>
                <pre className="whitespace-pre-wrap">
                    testing env variable: {import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}
                </pre>
            </div>
        </>
    );
}
