use super::TemplateFile;

const PACKAGE_JSON: &str = include_str!("../../templates/react/package.json");

const VITE_CONFIG: &str = include_str!("../../templates/react/vite.config.ts");

const ESLINT_CONFIG: &str = include_str!("../../templates/react/eslint.config.js");

const PNPM_LOCK: &str = include_str!("../../templates/react/pnpm-lock.yaml");

const TSCONFIG: &str = include_str!("../../templates/react/tsconfig.json");
const TSCONFIG_APP: &str = include_str!("../../templates/react/tsconfig.app.json");
const TSCONFIG_NODE: &str = include_str!("../../templates/react/tsconfig.node.json");

const INDEX_HTML: &str = include_str!("../../templates/react/index.html");

const MAIN_TSX: &str = include_str!("../../templates/react/src/main.tsx");

const APP_TSX: &str = include_str!("../../templates/react/src/App.tsx");

const INDEX_CSS: &str = include_str!("../../templates/react/src/index.css");

const APP_CSS: &str = include_str!("../../templates/react/src/App.css");

const GITIGNORE: &str = include_str!("../../templates/react/.gitignore");

const ENV_EXAMPLE: &str = include_str!("../../templates/react/.env.example");

const DEVCONTAINER: &str = include_str!("../../templates/react/.devcontainer/devcontainer.json");

pub fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: "package.json",
            content: PACKAGE_JSON,
        },
        TemplateFile {
            path: "vite.config.ts",
            content: VITE_CONFIG,
        },
        TemplateFile {
            path: "tsconfig.json",
            content: TSCONFIG,
        },
        TemplateFile {
            path: "tsconfig.node.json",
            content: TSCONFIG_NODE,
        },
        TemplateFile {
            path: "tsconfig.app.json",
            content: TSCONFIG_APP,
        },
        TemplateFile {
            path: "eslint.config.js",
            content: ESLINT_CONFIG,
        },
        TemplateFile {
            path: "pnpm-lock.yaml",
            content: PNPM_LOCK,
        },
        TemplateFile {
            path: "index.html",
            content: INDEX_HTML,
        },
        TemplateFile {
            path: "src/main.tsx",
            content: MAIN_TSX,
        },
        TemplateFile {
            path: "src/App.tsx",
            content: APP_TSX,
        },
        TemplateFile {
            path: "src/App.css",
            content: APP_CSS,
        },
        TemplateFile {
            path: "src/index.css",
            content: INDEX_CSS,
        },
        TemplateFile {
            path: ".gitignore",
            content: GITIGNORE,
        },
        TemplateFile {
            path: ".env.example",
            content: ENV_EXAMPLE,
        },
        TemplateFile {
            path: ".devcontainer/devcontainer.json",
            content: DEVCONTAINER,
        },
    ]
}
