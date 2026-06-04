
pub mod react;
pub mod express;
pub mod hono;
pub mod next;

pub struct TemplateFile {

    pub path: &'static str,

    pub content: &'static str
}

pub const GITIGNORE: &str = r#"node_modules/
dist/
.env
.DS_Store
*.log"#;

pub const ENV_EXAMPLE: &str = r#"# App
PORT=3000
NODE_ENV=development"#;


