import {
    type RouteConfig,
    route,
} from "@react-router/dev/routes";
import { flatRoutes } from "@react-router/fs-routes";

// file based routing using specified directory
export default flatRoutes({
    rootDirectory: "pages"
}) satisfies RouteConfig;

// original way when using react-router
//export default [
//    // * matches all URLs, the ? makes it optional so it will match / as well
//    route("*?", "catchall.tsx"),
//    route("/about", "./pages/about.tsx"),
//    route("/logs", "./pages/logs.tsx"),
//] satisfies RouteConfig;
