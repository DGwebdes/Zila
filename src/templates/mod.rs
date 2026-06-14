pub mod express;
pub mod hono;
pub mod react;

pub struct TemplateFile {
    pub path: &'static str,

    pub content: &'static str,
}
