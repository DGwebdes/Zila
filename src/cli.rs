
pub enum Stack {
    Frontend,
    Backend,
    Fullstack
}

pub enum Framework {
    React,
    Express,
    Hono,
    Next
}

pub enum PackageManager {
    Npm,
    Pnpm,
    Bun,
}

pub struct ProjectConfig {
    pub name: String,
    pub framework: Framework,
    pub package_manager: PackageManager,
}

