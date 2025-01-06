import { getApiUrl } from "@/lib/useApiUrl";

export const doSystemRestart = async () => {
    const req_body: string = JSON.stringify({})

    try {

        const response = await fetch(`${getApiUrl()}/system/restart`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();
    }
    catch (e) {
        console.error(e)
    };
}

export const ResetButton = () => {

    return (
        <>
            <span
                onClick={async() => {
                    await doSystemRestart()

                }}          
                className="hover:cursor-pointer w-fit px-2.5 py-1.5 rounded-md bg-secondary text-white hover:underline">HW System Restart</span>
        </>
    )
}
