
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::cli::{Framework, PackageManager, ProjectConfig};
use crate::templates::{self, TemplateFile};

pub fn run(config: &ProjectConfig) -> Result<(), anyhow::Error> {

    let files = match config.framework {
        Framework::React => crate::templates::react::files(),
        Framework::Express => crate::templates::express::files(),
        Framework::Hono => crate::templates::hono::files(),
    };

    let root = Path::new(&config.name);

    if root.exists() {
        return Err(anyhow::anyhow!(
                "Directory '{}' already exists", config.name    
            ));
    }

    fs::create_dir(&root)?;

    println!("\n Creating {} ...", config.name);

    for file in &files {

        let content = file.content.replace("{{name}}", &config.name);

        let full_path = root.join(file.path);

        if let Some(parent) = full_path.parent() {

            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, content)?;

        println!(" Done {}", file.path);
    }


    println!("\n Initializing git...");

    Command::new("git")
        .args(["init"])
        .current_dir(&root)
        .output()?;

    println!(" Done git init");

    let pm = match config.package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    };

    println!("\n Installing dependencies with {}...", pm);

    let status = Command::new(pm)
        .args(["install"])
        .current_dir(&root)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("{} install failed", pm));
    }

    println!("\n Godzillaed your project successfully!\n");
    println!(" cd {}", config.name);
    println!(" {} run dev\n", pm);

    Ok(())

}
