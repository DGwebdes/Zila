use super::TemplateFile;

const PACKAGE_JSON: &str = include_str!("../../templates/express/package.json");
const TSCONFIG: &str = include_str!("../../templates/express/tsconfig.json");
const DEVCONTAINER: &str = include_str!("../../templates/express/.devcontainer/devcontainer.json");

const INDEX_TS: &str = include_str!("../../templates/express/src/index.ts");
const APP_TS: &str = include_str!("../../templates/express/src/app.ts");
const ROUTES_INDEX: &str = include_str!("../../templates/express/src/routes/index.ts");
const CONTROLLERS_INDEX: &str = include_str!("../../templates/express/src/controllers/index.ts");
const LIB_CONFIG: &str = include_str!("../../templates/express/src/lib/config.ts");
const DB_CONFIG: &str = include_str!("../../templates/express/src/lib/db.ts");

const ERROR_HANDLER: &str = include_str!("../../templates/express/src/middleware/errorHandler.ts");
const NOT_FOUND: &str = include_str!("../../templates/express/src/middleware/notFound.ts");
const LOGGER: &str = include_str!("../../templates/express/src/middleware/logger.ts");

const GITIGNORE: &str = include_str!("../../templates/express/.gitignore");
const ENV_EXAMPLE: &str = include_str!("../../templates/express/.env.example");

pub fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            path: "package.json",
            content: PACKAGE_JSON,
        },
        TemplateFile {
            path: "tsconfig.json",
            content: TSCONFIG,
        },
        TemplateFile {
            path: "src/index.ts",
            content: INDEX_TS,
        },
        TemplateFile {
            path: "src/app.ts",
            content: APP_TS,
        },
        TemplateFile {
            path: "src/routes/index.ts",
            content: ROUTES_INDEX,
        },
        TemplateFile {
            path: "src/controllers/index.ts",
            content: CONTROLLERS_INDEX,
        },
        TemplateFile {
            path: "src/lib/config.ts",
            content: LIB_CONFIG,
        },
        TemplateFile {
            path: "src/lib/db.ts",
            content: DB_CONFIG,
        },
        TemplateFile {
            path: "src/middleware/errorHandler.ts",
            content: ERROR_HANDLER,
        },
        TemplateFile {
            path: "src/middleware/notFound.ts",
            content: NOT_FOUND,
        },
        TemplateFile {
            path: "src/middleware/logger.ts",
            content: LOGGER,
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
