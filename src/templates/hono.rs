
use super::TemplateFile;
use super::{GITIGNORE, ENV_EXAMPLE};

const PACKAGE_JSON: &str = r#"{
  "name": "{{name}}",
  "version": "0.0.1",
  "type": "module",
  "scripts": {
    "dev": "tsx watch src/index.ts",
    "build": "tsc",
    "start": "node dist/index.js"
  },
  "dependencies": {
    "hono": "^4.12.23",
    "@hono/node-server": "^2.0.4"
  },
  "devDependencies": {
    "@types/node": "^25.9.1",  
    "typescript": "^6.0.3",
    "tsx": "^4.22.4"
  }
}"#;

const TSCONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "skipLibCheck": true,
    "esModuleInterop": true
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
}"#;

const INDEX_TS: &str = r#"import { serve } from '@hono/node-server'
import app from './app.js'

const PORT = Number(process.env.PORT) || 3000

serve({ fetch: app.fetch, port: PORT }, () => {
  console.log(`Server running on port ${PORT}`)
})"#;

const APP_TS: &str = r#"import { Hono } from 'hono'
import { cors } from 'hono/cors'
import { secureHeaders } from 'hono/secure-headers'
import { router } from './routes/index.js'

const app = new Hono()

app.use('*', cors())
app.use('*', secureHeaders())

app.route('/api', router)

export default app"#;

const ROUTES_INDEX: &str = r#"import { Hono } from 'hono'
import { getAll, getOne, create, update, remove } from '../controllers/index.js'

export const router = new Hono()

router.get('/',       getAll)
router.get('/:id',    getOne)
router.post('/',      create)
router.put('/:id',    update)
router.delete('/:id', remove)"#;

const CONTROLLERS_INDEX: &str = r#"import type { Context } from 'hono'

export function getAll(c: Context) {
  return c.json({ message: 'getAll' })
}

export function getOne(c: Context) {
  return c.json({ message: 'getOne', id: c.req.param('id') })
}

export function create(c: Context) {
  return c.json({ message: 'created' }, 201)
}

export function update(c: Context) {
  return c.json({ message: 'updated', id: c.req.param('id') })
}

export function remove(c: Context) {
  return c.json({ message: 'deleted', id: c.req.param('id') })
}"#;

const DEVCONTAINER: &str = r#"{
  "name": "{{name}}",
  "image": "mcr.microsoft.com/devcontainers/typescript-node:24",
  "forwardPorts": [3000],
  "postCreateCommand": "npm install",
  "customizations": {
    "vscode": {
      "extensions": [
        "dbaeumer.vscode-eslint"
      ]
    }
  }
}"#;

pub fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json",                    content: PACKAGE_JSON       },
        TemplateFile { path: "tsconfig.json",                   content: TSCONFIG           },
        TemplateFile { path: "src/index.ts",                    content: INDEX_TS           },
        TemplateFile { path: "src/app.ts",                      content: APP_TS             },
        TemplateFile { path: "src/routes/index.ts",             content: ROUTES_INDEX       },
        TemplateFile { path: "src/controllers/index.ts",        content: CONTROLLERS_INDEX  },
        TemplateFile { path: ".gitignore",                      content: GITIGNORE          },
        TemplateFile { path: ".env.example",                    content: ENV_EXAMPLE        },
        TemplateFile { path: ".devcontainer/devcontainer.json", content: DEVCONTAINER       },
    ]
}

