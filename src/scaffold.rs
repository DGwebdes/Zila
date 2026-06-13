
use std::fs;
use std::path::Path;
use std::process::Command;
use colored::Colorize;
use crate::cli::{Framework, PackageManager, ProjectConfig};
// use crate::templates::TemplateFile;


pub fn run(config: &ProjectConfig) -> Result<(), anyhow::Error> {


    let pm = match config.package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    };

    let pm_url = match config.package_manager {
        PackageManager::Npm => "https://nodejs.org",
        PackageManager::Pnpm => "https://pnpm.io",
        PackageManager::Bun => "https://bun.sh",
    };

    check_node_version()?;


    if !command_exists("git") {
        return Err(anyhow::anyhow!(
                "git is not installed or not in PATH.\n Install it from https://git-scm.com"
        ));
    }

    if !command_exists(pm) {
        return Err(anyhow::anyhow!(
                "'{}' is not installed or not in PATH.\n install it from {}",
                pm,
                pm_url
        ));
    }


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

    println!("\n {}", format!("Creating {}...", config.name).cyan().bold());

    for file in &files {

        let content = file.content.replace("{{name}}", &config.name);

        let full_path = root.join(file.path);

        if let Some(parent) = full_path.parent() {

            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, content)?;

        println!(" {} {}", "Done".green(), file.path.dimmed());
    }


    println!("\n {}", "Initializing git...".cyan());

    Command::new("git")
        .args(["init"])
        .current_dir(&root)
        .output()?;

    println!(" {} git init", "Done".green());

    println!("\n {}", format!("Installing dependencies with {}...", pm).cyan());

    let status = Command::new(pm)
        .args(["install"])
        .current_dir(&root)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("{} install failed", pm));
    }

    println!("\n {}\n", "Godzillaed your project successfully!\n".green().bold());
    println!(" {}", format!("cd {}", config.name).yellow());
    println!(" {}\n", format!("{} run dev", pm).yellow());

    Ok(())

}


fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn check_node_version() -> Result<(), anyhow::Error> {
    if !command_exists("node") {
        return Err(anyhow::anyhow!(
                "{}",
                "Nodejs is not installed.\n Install it from https://nodejs.org".red()
        ));
    }

    let output = Command::new("node")
        .arg("--version")
        .output()?;

    let version_str = String::from_utf8(output.stdout)?;

    let version_str = version_str.trim().trim_start_matches('v');

    let parts: Vec<u64> = version_str
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    let major = parts.first().copied().unwrap_or(0);

    if major < 26 {
        return Err(anyhow::anyhow!(
                    "Nodejs >= 26.2.0 is required. You ave v{}.\n Update at https://nodejs.org",
                    version_str
        ));
    }

    Ok(())
}
