import { defineConfig } from 'vite'
//import react from '@vitejs/plugin-react-swc'
import { reactRouter } from "@react-router/dev/vite";
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        //   react()
        reactRouter()
    ],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src')
        }
    }
})
