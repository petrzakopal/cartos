import { SpaceY } from "@/components/space";
import { Button } from "@/components/ui/button";
import { NavLink } from "react-router";

export const DesktopNavigation = () => {
    return (
        <>
            <SpaceY mt="mt-6" />
            <div className="w-full flex flex-row space-x-5 relative items-start">
                <DesktopNavigationLink href="/" text="Home" />
                <DesktopNavigationLink href="/logs" text="Logs" />
                <DesktopNavigationLink href="/users" text="Users" />
                <DesktopNavigationLink href="/about" text="About" />
            </div>

            <SpaceY mt="mt-5" />
        </>
    )
}

const DesktopNavigationLink = ({ href, text }: { href: string, text: string }) => {

    return (
        <>
            <NavLink className={"w-fit"} to={href} >
                <Button className="w-fit text-black bg-slate-200 hover:bg-slate-300 ease-in-out transition-all">
                    {text}
                </Button>
            </NavLink>

        </>
    )
}

export const PageTitle = ({ text }: { text: string }) => {
    return (
        <>
            <h1 className="text-2xl font-bold text-primary">{text}</h1>
        </>
    )
}
