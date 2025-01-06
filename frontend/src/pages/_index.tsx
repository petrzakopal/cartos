import { Helmet } from "react-helmet"
import { NavLink } from "react-router"
function App() {

    return (
        <>
            <Helmet>
                <title>Main page</title>
            </Helmet>
            <div className='w-full relative flex flex-col items-center'>
                <h1 className='text-black text-4xl font-bold'>Cartos</h1>
                <span className="italic text-primary">under development till further notice</span>

                <p>If there are any problems with the application contact the maintainers of the repository at <NavLink className="italic text-primary" to={"https://github.com/petrzakopal/cartos"}>petrzakopal/cartos</NavLink>.</p>
            </div>
        </>
    )
}

export default App
