

use super::TemplateFile;
use super::{GITIGNORE, ENV_EXAMPLE};

const PACKAGE_JSON: &str = r#"{
    "name": "{{name}}",
    "private": true,
    "version": "0.0.1",
    "type": "module",
    "scripts": {
      "dev": "vite",
      "build": "tsc && vite build",
      "preview": "vite preview"
    },
    "dependencies": {
      "react": "^19.2",
      "react-dom": "^19.2.7",
      "react-router-dom": "^7.17.0"
    },
    "devDependencies": {
      "@types/react": "^19.2.16",
      "@types/react-dom": "^19.2.3",
      "@vitejs/plugin-react": "^6.0.2",
      "typescript": "^6.0.3",
      "vite": "^8.0.16",
      "tailwindcss": "^4.3.0",
      "tailwindcss/vite": "^4.3.0",
      "postcss": "^8.5.15",
      "autoprefixer", "^10.5.0",
      "eslint": "^10.4.1",
      "@typescript-eslint/eslint-plugin": "^8.60.1"
    }
}"#;


const VITE_CONFIG: &str = r#"import { defineConfig } from 'vite'
import path from 'path'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
    plugins: [react(), tailwindcss()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src')
        }
    }
})"#;

const TSCONFIG: &str = r#"{
    "compilerOptions": {
        "target": "ES2020",
        "useDefineForClassFields": true,
        "lib": ["ES2020", "DOM", "DOM.Iterable"],
        "module": "ESNext",
        "skipLibCheck": true,
        "moduleResolution": "bundler",
        "allowImportingTsExtensions": true,
        "isolatedDetection": true,
        "moduleDetection": "force",
        "noEmit": true,
        "jsx": "react-jsx",
        "strict": true,
        "paths": {
            "@/*": ["./src/*"]
        }
    },
    "include": ["src"]
}"#;

const TAILWIND_CONFIG: &str = r#"import type { Config } from 'tailwindcss"

export default {
    content: [
        './index.html',
        './src/**/*.{ts,tsx}',
    ],
    theme: {
        extend: {},
    },
    plugins: [],
} satisfies Config"#;

const POSTCSS_CONFIG:  &str = r#"export default {
    plugins: {
        tailwindcss: {},
        autoprefixer: {},
    },
}"#;

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>{{name}}</title>
    </head>
    <body>
        <div id="root"></div>
        <script type="module" src="/src/main.tsx"></script>
    </body>
</html>"#;

const MAIN_TSX: &str = r#"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App'

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>
)"#;

const APP_TSX: &str = r#"function App() {
    return (
        <main className="min-h-screen bg-white>
            <h1 className="text-2xl font-bold"> Hello from Zila </h1>
        </main>
    )
}

export default App"#;

const INDEX_CSS: &str = r#"@tailwind base;
@tailwind components;
@tailwind utilities;"#;

const DEVCONTAINER: &str = r#"{
    "name": "{{name}}",
    "image": "mcr.microsoft.com/devcontainers/typescript-node:24",
    "forwardPorts": [5173],
    "postCreateCommand": "pnpm install",
    "customizations": {
        "vscode": {
            "extensions": [
                "dbaeumer.vscode-eslint",
                "bradlc.vscode-tailwindcss",
            ]
        }
    }
}"#;

pub fn files() -> Vec<TemplateFile> {

    vec![
        TemplateFile { path: "package.json",        content: PACKAGE_JSON},
        TemplateFile { path: "vite.config.ts",      content: VITE_CONFIG},
        TemplateFile { path: "tsconfig.json",       content: TSCONFIG},
        TemplateFile { path: "tailwind.config.ts",  content: TAILWIND_CONFIG},
        TemplateFile { path: "postcss.config.js",   content: POSTCSS_CONFIG},
        TemplateFile { path: "index.html",          content: INDEX_HTML},
        TemplateFile { path: "src/main.tsx",        content: MAIN_TSX},
        TemplateFile { path: "src/App.tsx",         content: APP_TSX},
        TemplateFile { path: "src/index.css",       content: INDEX_CSS},
        TemplateFile { path: ".gitignore",          content: GITIGNORE},
        TemplateFile { path: ".env.example",        content: ENV_EXAMPLE},
        TemplateFile { path: ".devcontainer/devcontainer.json", content: DEVCONTAINER},
    ]
}
