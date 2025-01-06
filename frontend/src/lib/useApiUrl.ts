export const getApiUrl = () => {
    const environment = import.meta.env.VITE_ENVIRONMENT;
    const port = import.meta.env.VITE_BACKEND_API_PORT;
    let API_BASE_URL = "";
    switch (environment) {
        case "production": {
            API_BASE_URL = `http://${window.location.hostname}:${port}`;
            break;
        }
        case "development": {

            API_BASE_URL = import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND;
            break;
        }
        default: {
            API_BASE_URL = import.meta.env.VITE_BACKEND_URL_VISIBLE_IN_THE_FRONTEND;
            break;
        }
    }

    return API_BASE_URL;
}
