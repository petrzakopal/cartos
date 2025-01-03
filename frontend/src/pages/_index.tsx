import { Helmet } from "react-helmet"
function App() {

    return (
        <>
            <Helmet>
                <title>Main page</title>
            </Helmet>
            <div className='w-full relative flex flex-col items-center'>
                <h1 className='text-black font-bold'>Cartos</h1>
                <span className="italic text-primary">under development</span>
            </div>
        </>
    )
}

export default App
