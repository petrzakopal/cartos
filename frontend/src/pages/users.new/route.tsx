import { PageTitle } from "@/components/navigation";
import { SpaceY } from "@/components/space";
import { addNewUserEntry, FormSectionWithHeading, FormSelectInput, FormTextAreaInput, FormTextInput, TwoColumnFormInput, updateUserEntry, userActivityStatusOptions, UserEntry, UserStatus } from "@/components/users/editForm/formComponents";
import { useEffect, useState } from "react";
import { SubmitHandler, useForm, UseFormRegister } from "react-hook-form";


import { useLocation, useNavigate } from 'react-router-dom'


type LoaderData = {
    title: string;
};


export async function clientLoader() {
        return {
            title: "Add new user",
        };
}


export default function Component({ loaderData }: { loaderData: LoaderData }) {
    const location = useLocation()


    const navigate = useNavigate();
    const { register, handleSubmit, reset, formState } = useForm<UserEntry>();

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


        addNewUserEntry(data).then((data)=>{
            console.log("added new user", data);
        });
    }

        return (
            <>
                <div className="w-full flex flex-col">
                    <PageTitle text={loaderData.title} />
                    <SpaceY mt="mt-12" />
                    <form onSubmit={handleSubmit(onSubmit)} className="w-full relative flex flex-col">
                        <FormSectionWithHeading heading={"User details"}>
                            <TwoColumnFormInput>
                                <FormTextInput
                                    id="card_serial_number"
                                    type="text"
                                    name="card_serial_number"
                                    label="Card Serial Number"
                                    placeholder={"AA:BB:11:22"}
                                    isRequired={true}
                                    registerFce={{ ...register("card_serial_number") }}
                                />
                                <FormTextInput
                                    id="email"
                                    type="text"
                                    name="email"
                                    label="Email"
                                    placeholder={"hi@example.com"}
                                    isRequired={true}
                                    registerFce={{ ...register("email") }}
                                />


                                <FormTextAreaInput
                                    id="note"
                                    name="note"
                                    label="Note"
                                    rows={3}
                                    //placeholder={usrData.note}
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
                                    //defaultValue={"active"}
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
                                    Add new user
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </>
        );

    }
