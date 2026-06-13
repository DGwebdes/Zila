
use super::TemplateFile;

const PACKAGE_JSON: &str = include_str!("../../templates/hono/package.json");
const PNPM_LOCK: &str = include_str!("../../templates/hono/pnpm-lock.yaml");
const PNPM_WORKSPACE: &str = include_str!("../../templates/hono/pnpm-workspace.yaml");
const TSCONFIG: &str = include_str!("../../templates/hono/tsconfig.json");
const README: &str = include_str!("../../templates/hono/README.md");

const APP_TS: &str = include_str!("../../templates/hono/src/app.ts");
const INDEX_TS: &str = include_str!("../../templates/hono/src/index.ts");

const CONTROLLERS_INDEX: &str = include_str!("../../templates/hono/src/controllers/index.ts");
const ROUTES_INDEX: &str = include_str!("../../templates/hono/src/routes/index.ts");
const DEVCONTAINER: &str = include_str!("../../templates/hono/.devcontainer/devcontainer.json");

const GITIGNORE: &str = include_str!("../../templates/hono/.gitignore");
const ENV_EXAMPLE: &str = include_str!("../../templates/hono/.env.example");


pub fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json",                    content: PACKAGE_JSON       },
        TemplateFile { path: "tsconfig.json",                   content: TSCONFIG           },
        TemplateFile { path: "pnpm-locl.yaml",                  content: PNPM_LOCK          },
        TemplateFile { path: "pnpm-workspace.yaml",             content: PNPM_WORKSPACE     },
        TemplateFile { path: "README.md",                       content: README             },
        TemplateFile { path: "src/index.ts",                    content: INDEX_TS           },
        TemplateFile { path: "src/app.ts",                      content: APP_TS             },
        TemplateFile { path: "src/routes/index.ts",             content: ROUTES_INDEX       },
        TemplateFile { path: "src/controllers/index.ts",        content: CONTROLLERS_INDEX  },
        TemplateFile { path: ".gitignore",                      content: GITIGNORE          },
        TemplateFile { path: ".env.example",                    content: ENV_EXAMPLE        },
        TemplateFile { path: ".devcontainer/devcontainer.json", content: DEVCONTAINER       },
    ]
}

