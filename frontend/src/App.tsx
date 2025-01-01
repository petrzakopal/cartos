import { NavLink } from "react-router";
function App() {

    return (
        <>
            <div className='w-full relative flex flex-col items-center'>
                <h1 className='text-black font-bold'>Cartos</h1>
                <span className="italic text-blue-400">under development</span>
                <NavLink to={"/about"}>To about</NavLink>
            </div>
        </>
    )
}

export default App
