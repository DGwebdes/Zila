use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(
    name = "zila",
    about = "a god, if you will, of scaffolding projects",
    version = "0.1.0"
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    New { name: String },
}

pub enum Framework {
    React,
    Express,
    Hono,
}

pub enum PackageManager {
    Npm,
    Pnpm,
    Bun,
}

#[allow(dead_code)]
pub struct ProjectConfig {
    pub name: String,
    pub framework: Framework,
    pub package_manager: PackageManager,
}
