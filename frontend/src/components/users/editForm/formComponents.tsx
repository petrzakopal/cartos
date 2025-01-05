import { ctw } from "@/lib/ctwHelper";

export type UserStatusValues = "active" | "deactivated";

export type UserStatus = {
    value: UserStatusValues,
    displayValue: string
}


export type UserEntry = {
    id: number,
    card_serial_number: string,
    email: string,
    note: string | undefined,
    updated_at: string,
    status: UserStatusValues
}

export const FormTextInput = ({
    id,
    type,
    name,
    label,
    placeholder,
    additional,
    isLabel,
    registerFce,
    isRequired,
    defaultValue
}: {
    id: any;
    type: string;
    name?: string;
    label: any;
    placeholder?: string;
    additional?: any;
    isLabel?: boolean;
    registerFce?: any;
    isRequired?: any;
    defaultValue?: string;
}) => {
    return (
        <>
            <div className="grid grid-cols-1 gap-y-1">
                {isLabel == false ? (
                    ""
                ) : (
                    <>
                        <label htmlFor={id} className={ctw("text-dark-black dark:text-white-lightest font-bold text-lg", isRequired ? "after:content-['*'] after:text-red-500 after:ml-0.5" : "")}>
                            {label}
                        </label>
                    </>
                )}

                <input
                    id={id}
                    type={type}
                    name={name}
                    defaultValue={defaultValue}
                    className="bg-white text-dark-black dark:text-dark-black rounded-xl font-bold text-base py-2.5 px-3 border border-solid border-neutral-300"
                    placeholder={placeholder}
                    {...registerFce}
                    required={isRequired}
                />
            </div>
        </>
    );
};



export const FormTextAreaInput = ({
    id,
    name,
    label,
    placeholder,
    additional,
    isLabel,
    registerFce,
    isRequired,
    defaultValue,
    rows
}: {
    id: any;
    name?: string;
    label: any;
    placeholder?: string;
    additional?: any;
    isLabel?: boolean;
    registerFce?: any;
    isRequired?: any;
    defaultValue?: string;
    rows: number | 3
}) => {
    return (
        <>
            <div className="grid grid-cols-1 gap-y-1">
                {isLabel == false ? (
                    ""
                ) : (
                    <>
                        <label htmlFor={id} className={ctw("text-dark-black dark:text-white-lightest font-bold text-lg", isRequired ? "after:content-['*'] after:text-red-500 after:ml-0.5" : "")}>
                            {label}
                        </label>
                    </>
                )}

                <textarea
                    id={id}
                    name={name}
                    defaultValue={defaultValue}
                    className="bg-white text-dark-black dark:text-dark-black rounded-xl font-bold text-base py-2.5 px-3 border border-solid border-neutral-300"
                    placeholder={placeholder}
                    {...registerFce}
                    required={isRequired}
                />
            </div>
        </>
    );
};


export const FormSelectInput = ({
    id,
    name,
    label,
    isLabel,
    registerFce,
    isRequired,
    defaultValue,
    options
}: {
    id: any;
    name?: string;
    label: any | string;
    isLabel?: boolean;
    registerFce?: UseFormRegister<any>;
    isRequired?: boolean;
    defaultValue?: string;
    options: any

}) => {
    return (
        <>
            <div className="grid grid-cols-1 gap-y-1">
                {isLabel == false ? (
                    ""
                ) : (
                    <>
                        <label htmlFor={id} className={ctw("text-dark-black dark:text-white-lightest font-bold text-lg", isRequired ? "after:content-['*'] after:text-red-500 after:ml-0.5" : "")}>
                            {label}
                        </label>
                    </>
                )}

                <select
                    id={id}
                    name={name}
                    className="bg-white text-dark-black dark:text-dark-black rounded-xl font-bold text-base py-2.5 px-3 border border-solid border-neutral-300"
                    {...registerFce}
                    required={isRequired}
                >
                    {/*<option value="">{defaultValue}</option>*/}

                    {options.map((status) => {
                        return (
                            <option key={status.value} value={status.value}>{status.displayValue}</option>
                        )
                    })}

                </select>
            </div>
        </>
    );
};


export const TwoColumnFormInput = ({ children }: { children: React.ReactNode }) => {
    return <div className="grid grid-cols-2 gap-x-8 gap-y-8">{children}</div>;
};

export const FormSectionWithHeading = ({
    children,
    heading,
}: {
    children: React.ReactNode;
    heading: string;
}) => {
    return (
        <div className="w-full grid grid-cols-1 gap-y-2">
            <h2 className="text-dark-black dark:text-white-lightest font-bold text-xl">{heading}</h2>
            {children}
        </div>
    );
};


export const updateUserEntry = async (userData: UserEntry) => {

    const req_formatted = {

        id: userData.id,
        email: userData.email,
        card_data: {
            serial_number: userData.card_serial_number
        },
        status: userData.status,
        note: userData.note
    };


    const req_body: string = JSON.stringify(req_formatted)
    try {

        const response = await fetch(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/user/update`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: req_body
        });
        const data = await response.json();

        return data;

    }
    catch (e) {
        console.error(e)
        return null;
    };
}
