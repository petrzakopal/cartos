// server-ssr.js
import express from 'express';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
import compression from 'compression';
import serveStatic from 'serve-static';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const app = express();
const port = process.env.PORT || 3000;

// Enable gzip compression
app.use(compression());

// Serve static assets with caching
app.use(
  '/assets',
  serveStatic(resolve(__dirname, 'build/client/assets'), {
    index: false,
    immutable: true,
    maxAge: '1y'
  })
);

// Serve all static files
app.use(serveStatic(resolve(__dirname, 'build/client')));

// Import the SSR entry point
const ssrModule = await import('./build/server/index.js');
const render = ssrModule.default;

app.use('*', async (req, res) => {
  try {
    // Get the URL from the request
    const url = req.originalUrl;

    // Render the app
    const { html, state } = await render(url);

    // Create a simple HTML shell with the rendered content
    const response = `
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <title>Vite SSR App</title>
          <script>window.__INITIAL_STATE__ = ${JSON.stringify(state)}</script>
        </head>
        <body>
          <div id="root">${html}</div>
        </body>
      </html>
    `.trim();

    res.status(200).set({ 'Content-Type': 'text/html' }).end(response);
  } catch (e) {
    console.error(e);
    res.status(500).end(e.stack);
  }
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
