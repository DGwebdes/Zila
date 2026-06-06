use dialoguer::{Select, Confirm};
use dialoguer::theme::ColorfulTheme;
use super::cli::{Framework, PackageManager, ProjectConfig};

pub fn ask(name: String) -> Result<ProjectConfig, anyhow::Error> {
    let theme = ColorfulTheme::default();

    let frameworks = vec![
        "React      (Vite + Typescript + Tailwindcss)",
        "Express    (Typescript REST API)",
        "Hono       (Typescript, lightweight and fast)",
        "Vue        (Vue + Typescript + Eslint)",
    ];

    let framework_index = Select::with_theme(&theme)
        .with_prompt("What are you building?")
        .items(&frameworks)
        .default(0)
        .interact()?;

    let framework = match framework_index {
        0 => Framework::React,
        1 => Framework::Express,
        2 => Framework::Hono,
        3 => Framework::Next,
        _ => Framework::React,
    };


    let package_managers = vec!["npm", "pnpm", "bun"];

    let pm_index = Select::with_theme(&theme)
        .with_prompt("Who's the manager?")
        .items(&package_managers)
        .default(0)
        .interact()?;

    let package_manager = match pm_index {
        0 => PackageManager::Npm,
        1 => PackageManager::Pnpm,
        2 => PackageManager::Bun,
        _ => PackageManager::Pnpm
    };

    

    let framework_name = match framework {
        Framework::React => "React + Vite + Typescript + Tailwindcss",
        Framework::Express => "Express + Typescript",
        Framework::Hono => "Hono + Typescript",
        Framework::Next => "Nextjs + Typescript + Tailwindcss",
    };

    let pm_name = match package_manager {
        PackageManager::Npm => "npm",
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    };

    println!("\n Project: {}", name);
    println!(" Stack: {}", framework_name);
    println!(" Manager: {}", pm_name);

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

