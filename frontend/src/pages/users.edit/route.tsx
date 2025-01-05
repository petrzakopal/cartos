import { PageTitle } from "@/components/navigation";
import { SpaceY } from "@/components/space";
import { UserEntry } from "@/components/users/columns";
import { FormSectionWithHeading, FormSelectInput, FormTextAreaInput, FormTextInput, TwoColumnFormInput, updateUserEntry, UserStatus } from "@/components/users/editForm/formComponents";
import { useEffect, useState } from "react";
import { SubmitHandler, useForm, UseFormRegister } from "react-hook-form";

import { data } from "react-router";

import { useLocation, useNavigate } from 'react-router-dom'


type LoaderData = {
    users: [];
    title: string;
};


const userActivityStatusOptions: UserStatus[] = [
    {
        value: "active",
        displayValue: "Active"
    },
    {
        value: "deactivated",
        displayValue: "Deactivated"
    }
]


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
            title: "Edit user",
            users: data
        };
    }
    catch (e) {
        console.error(e)
        return {
            title: "Edit user",
            users: JSON.parse('{"msg":"no users"}')
        };
    };
}


export default function Component({ loaderData }: { loaderData: LoaderData }) {
    const location = useLocation()
    let usrDataFromLoader: UserEntry;
    if (location.state && location.state.usrData != null) {
        usrDataFromLoader = location.state.usrData
    } else {
        usrDataFromLoader = null // or set to a default value
    }


    const navigate = useNavigate();
    const { register, handleSubmit, reset, formState } = useForm<UserEntry>();
    const [usrData, setUsrData] = useState<UserEntry>(usrDataFromLoader);

    useEffect(() => {
        if (formState.isSubmitSuccessful) {
            reset({ id: 0, card_serial_number: "", email: "", note: "", updated_at: "", status: "" });

            navigate("/users")
            //setTimeout(() => {
            //    navigate("/users")
            //}, 0)

        }
    }, [formState, reset, navigate]);


    const onSubmit: SubmitHandler<UserEntry> = async (data) => {

        data.id = usrData.id

        updateUserEntry(data).then((data)=>{
            setUsrData(data)
        });

        //const req_from_form_data = {

        //    id: data.id,
        //    email: data.email,
        //    card_data: {
        //        serial_number: data.card_serial_number
        //    },
        //    status: data.status,
        //    note: data.note
        //};


        //const req_body: string = JSON.stringify(req_from_form_data)
        //try {

        //    const response = await fetch(`${import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND}/user/update`, {
        //        method: "POST",
        //        headers: {
        //            "Content-Type": "application/json"
        //        },
        //        body: req_body
        //    });
        //    const data = await response.json();

        //    setUsrData(data)

        //}
        //catch (e) {
        //    console.error(e)
        //    setUsrData(data);
        //};
    }

    let form_heading: string;
    if (typeof usrData.updated_at === "string" && usrData.updated_at.length === 0) {
        form_heading = "Editing user: " + usrData.email;
    }
    else {
        form_heading = "Editing user: " + usrData.email + " previously updated at: " + usrData.updated_at;
    }

    if (usrData != null) {
        return (
            <>
                <div className="w-full flex flex-col">
                    <PageTitle text={loaderData.title} />
                    <SpaceY mt="mt-12" />
                    <form onSubmit={handleSubmit(onSubmit)} className="w-full relative flex flex-col">
                        <FormSectionWithHeading heading={form_heading}>
                            <TwoColumnFormInput>
                                <FormTextInput
                                    id="card_serial_number"
                                    type="text"
                                    name="card_serial_number"
                                    label="Card Serial Number"
                                    placeholder={usrData.card_serial_number}
                                    defaultValue={usrData.card_serial_number}
                                    isRequired={true}
                                    registerFce={{ ...register("card_serial_number") }}
                                />
                                <FormTextInput
                                    id="email"
                                    type="text"
                                    name="email"
                                    label="Email"
                                    placeholder={usrData.email}
                                    defaultValue={usrData.email}
                                    isRequired={true}
                                    registerFce={{ ...register("email") }}
                                />


                                <FormTextAreaInput
                                    id="note"
                                    name="note"
                                    label="Note"
                                    rows={3}
                                    placeholder={usrData.note}
                                    defaultValue={usrData.note}
                                    isRequired={false}
                                    registerFce={{ ...register("note") }}
                                />

                                <FormSelectInput
                                    id={"status"}
                                    name="status"
                                    label={"Status"}
                                    isRequired={true}
                                    registerFce={{ ...register("status") }}
                                    options={userActivityStatusOptions}
                                    defaultValue={usrData.status}
                                />

                            </TwoColumnFormInput>
                        </FormSectionWithHeading>
                        <SpaceY mt="mt-10" />

                        <div className="w-full flex flex-col">
                            <div>
                                <button
                                    type="submit"
                                    className="order-1 md:order-2 bg-dark-black dark:bg-white text-white dark:text-dark-black hover:bg-dark-black-hover dark:hover:bg-white-lightest h-10 rounded-lg font-bold text-lg p-1.5 px-3 items-center justify-center uppercase w-full"
                                >
                                    Update the user
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </>
        );

    }
    else {
        return (
            <>
                <span className="text-secondary">No data from users page has been transfered.</span>
            </>
        )
    }
}
