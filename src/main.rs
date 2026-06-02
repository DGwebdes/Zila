

mod cli;
mod prompts;
mod scaffold;
mod templates;

use cli::{Framework, PackageManager, ProjectConfig};

fn main() {

    let config = ProjectConfig {
        name: String::from("my-app"),
        framework: Framework::React,
        package_manager: PackageManager::Npm
    };

    println!("Creating project: {}", config.name);
}
