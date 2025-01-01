# Frontend

Optimized for using [bun](https://bun.sh/).

## Deplyoment

### Static
When trying to server only static files, set `ssr:false` in [react-router.config.ts](react-router.config.ts) file
and after running `bun run build` do `bun server.js`.

And also at least the following code in the export default object must be present in the `react-router.config.ts`.

```js
import type { Config } from "@react-router/dev/config";

export default {
  appDirectory: "src",
  ssr: false,
} satisfies Config;
```

Also `loader()` function for SSR cannot be used in the components and routes.


### SSR

When trying to serve the app using SSR, set `ssr:true` in [react-router.config.ts](react-router.config.ts) file
and after running `bun run build` do `bun server-ssr.js`.

And also at least the following code in the export default object must be present in the `react-router.config.ts`.

```js
import type { Config } from "@react-router/dev/config";

export default {
  appDirectory: "src",
  ssr: true,
  async prerender() {
    return ["/"];
    }
} satisfies Config;
```


Also `loader()` function for SSR can be used in the components and routes.

```ts
// Define loader function only if SSR is disabled
export async function loader({ params }: Route.LoaderArgs) {
        console.log("server side variable BACKEND_URL:", process.env.BACKEND_URL);
}
```
