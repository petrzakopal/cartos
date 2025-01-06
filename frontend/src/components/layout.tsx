import { DesktopNavigation } from "@/components/navigation"
import { WebsocketDisplayer } from "./websockets/displayer"

/**
 * @brief Inner custom layout.
 */
export const LayoutInner = ({ children }: { children: React.ReactNode }) => {

    return (
        <>
            <div className="w-full overflow-hidden items-center flex flex-col relative">
                <main className="2xl:w-inside-full xl:w-inside-full-lite w-full relative flex flex-col items-center px-5 ">
                    <WebsocketDisplayer />
                    <DesktopNavigation />
                    {children}
                </main>
            </div>
        </>
    )
}
