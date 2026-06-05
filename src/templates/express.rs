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
    "express": "^5.2.1",
    "cors": "^2.8.6",
    "helmet": "^8.2.0"
  },
  "devDependencies": {
    "@types/express": "^5.0.6",
    "@types/cors": "^2.8.19",
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

const INDEX_TS: &str = r#"import app from './app.js'

const PORT = process.env.PORT ?? 3000

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`)
})"#;

const APP_TS: &str = r#"import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import { router } from './routes/index.js'

const app = express()

app.use(helmet())
app.use(cors())
app.use(express.json())

app.use('/api', router)

export default app"#;

const ROUTES_INDEX: &str = r#"import { Router } from 'express'
import { getAll, getOne, create, update, remove } from '../controllers/index.js'

export const router = Router()

router.get('/',       getAll)
router.get('/:id',    getOne)
router.post('/',      create)
router.put('/:id',    update)
router.delete('/:id', remove)"#;

const CONTROLLERS_INDEX: &str = r#"import type { Request, Response } from 'express'

export function getAll(req: Request, res: Response) {
  res.json({ message: 'getAll' })
}

export function getOne(req: Request, res: Response) {
  res.json({ message: 'getOne', id: req.params.id })
}

export function create(req: Request, res: Response) {
  res.status(201).json({ message: 'created', body: req.body })
}

export function update(req: Request, res: Response) {
  res.json({ message: 'updated', id: req.params.id })
}

export function remove(req: Request, res: Response) {
  res.json({ message: 'deleted', id: req.params.id })
}"#;

const DEVCONTAINER: &str = r#"{
  "name": "{{name}}",
  "image": "mcr.microsoft.com/devcontainers/typescript-node:22",
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
