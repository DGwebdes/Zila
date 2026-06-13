use super::TemplateFile;

pub fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json",                    content: PACKAGE_JSON   },
        TemplateFile { path: "next.config.ts",                  content: NEXT_CONFIG    },
        TemplateFile { path: "tsconfig.json",                   content: TSCONFIG       },
        TemplateFile { path: "tailwind.config.ts",              content: TAILWIND_CONFIG},
        TemplateFile { path: "src/app/layout.tsx",              content: LAYOUT_TSX     },
        TemplateFile { path: "src/app/page.tsx",                content: PAGE_TSX       },
        TemplateFile { path: "src/app/globals.css",             content: GLOBALS_CSS    },
        TemplateFile { path: ".gitignore",                      content: GITIGNORE      },
        TemplateFile { path: ".env.example",                    content: ENV_EXAMPLE    },
        TemplateFile { path: ".devcontainer/devcontainer.json", content: DEVCONTAINER   },
    ]
}
