import { DesktopNavigation } from "@/components/navigation"

/**
 * @brief Inner custom layout.
 */
export const LayoutInner = ({ children }: { children: React.ReactNode }) => {

    return (
        <>
            <div className="w-full overflow-hidden items-center flex flex-col relative">
                <main className="lg:w-inside-full w-full relative flex flex-col items-center px-5 ">
                    <DesktopNavigation />
                    {children}
                </main>
            </div>
        </>
    )
}
