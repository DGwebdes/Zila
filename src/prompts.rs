use super::cli::{Framework, PackageManager, ProjectConfig};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Select};

use colored::Colorize;

pub fn ask(name: String) -> Result<ProjectConfig, anyhow::Error> {
    let theme = ColorfulTheme::default();

    let frameworks = vec![
        "React      (Vite + Typescript + Tailwindcss)",
        "Express    (Typescript REST API)",
    ];

    let framework_index = Select::with_theme(&theme)
        .with_prompt("What are you building?")
        .items(&frameworks)
        .default(0)
        .interact()?;

    let framework = match framework_index {
        0 => Framework::React,
        1 => Framework::Express,
        _ => Framework::React,
    };

    let package_managers = vec!["npm", "pnpm"];

    let pm_index = Select::with_theme(&theme)
        .with_prompt("Who's the manager?")
        .items(&package_managers)
        .default(0)
        .interact()?;

    let package_manager = match pm_index {
        0 => PackageManager::Npm,
        1 => PackageManager::Pnpm,
        _ => PackageManager::Pnpm,
    };

    let framework_name = match framework {
        Framework::React => "React + Vite + Typescript + Tailwindcss",
        Framework::Express => "Express + Typescript",
    };

    let pm_name = match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
    };

    println!("\n {}", "Project Summary".cyan().bold());
    println!(" {} {}", "Project:".dimmed(), name.yellow());
    println!(" {} {}", "Stack:".dimmed(), framework_name.yellow());
    println!(" {} {}", "Manager:".dimmed(), pm_name.yellow());

    let confirmed = Confirm::with_theme(&theme)
        .with_prompt("Zila it?")
        .default(true)
        .interact()?;

    if !confirmed {
        return Err(anyhow::anyhow!("Aborted mission"));
    };

    Ok(ProjectConfig {
        name,
        framework,
        package_manager,
    })
}
