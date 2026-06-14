# zila 🦀

A fast, opinionated project scaffolder built in Rust. Yeet a project into existence.

## Requirements

- Node.js >= 24.16.0
- npm, pnpm, or bun

## Installation

```bash
cargo install zila
```

Or build from source:

```bash
git clone https://github.com/you/zila
cd zila
cargo build --release
```

## Usage

```bash
zila new <project-name>
```

Follow the interactive prompts to select your stack and package manager. Zila will scaffold a complete, ready-to-code project in seconds.

## What You Get

Every project includes:

- ✅ TypeScript configured and ready
- ✅ Opinionated folder structure
- ✅ Git initialized
- ✅ Dependencies installed
- ✅ `.env.example` with relevant variables
- ✅ `.devcontainer` for VS Code dev containers

## Supported Stacks

### Frontend

**React + Vite + TypeScript + Tailwind CSS**

```
my-app/
├── src/
│   ├── index.css
│   ├── App.css
│   ├── App.tsx
│   └── main.tsx
├── public/
├── index.html
├── vite.config.ts
├── tsconfig.json
├── .devcontainer/
├── .env.example
└── package.json
```

### Backend

**Express + TypeScript**

```
my-app/
├── src/
│   ├── routes/
│   ├── middleware/
│   ├── controllers/
│   ├── lib/
│   ├── app.ts
│   └── index.ts
├── tsconfig.json
├── .devcontainer/
├── .env.example
└── package.json
```

Includes: `cors`, `helmet`, basic CRUD route structure.

**Hono + TypeScript**

```
my-app/
├── src/
│   ├── routes/
│   ├── middleware/
│   ├── controllers/
│   ├── lib/
│   ├── app.ts
│   └── index.ts
├── tsconfig.json
├── .devcontainer/
├── .env.example
└── package.json
```

Includes: `cors`, `secureHeaders`, basic CRUD route structure.

## Roadmap

### v1 (current)
- [x] React + Vite + TypeScript + Tailwind
- [x] Express + TypeScript
- [x] Hono + TypeScript
- [x] Interactive prompts
- [x] Package manager selection (npm, pnpm, bun)
- [x] Git initialization
- [x] Devcontainer support

### v2
- [ ] Vue + Vite + TypeScript + Tailwind
- [ ] Next.js + TypeScript + Tailwind
- [ ] TanStack Start
- [ ] Database add-ons (PostgreSQL, MongoDB, Redis)
- [ ] Auth add-ons (Clerk)
- [ ] Remote templates via GitHub
- [ ] `zila add` command for adding integrations to existing projects

## Philosophy

Zila is opinionated by design. It reflects a specific, battle-tested workflow rather than trying to support every possible combination. If you need maximum flexibility, other tools exist. If you want to go from zero to coding in under a minute, zila is for you.

## Built With

- [Rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [dialoguer](https://github.com/console-rs/dialoguer) - Interactive prompts
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling

## License

MIT

## Author

DielanGarv
