import { NavLink } from "react-router";

export async function clientLoader() {
    // you can now fetch data here
    return {
        title: "About page",
    };
}

export default function Component({ loaderData }) {
    return (
        <>
            <div className="w-full flex flex-col">
                <h1>{loaderData.title}</h1>
                <span>test text about</span>
                <NavLink to={"/"} >To Home</NavLink>
            </div>
        </>
    );
}
