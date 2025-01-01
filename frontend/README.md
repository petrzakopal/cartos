# Frontend

Optimized for using [bun](https://bun.sh/).

## Deplyoment

### Static
When trying to server only static files, set `ssr:false` in [react-router.config.ts](react-router.config.ts) file
and after running `bun run build` do `bun server.js`.

### SSR

When trying to serve the app using SSR, set `ssr:true` in [react-router.config.ts](react-router.config.ts) file
and after running `bun run build` do `bun server-ssr.js`.
