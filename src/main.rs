mod cli;
mod prompts;
mod scaffold;
mod templates;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    if let Err(e) = run_app() {
        eprintln!("\n {}\n", format!("Error {}", e).red().bold());
        std::process::exit(1);
    }
}

fn run_app() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            println!("Zila summons {} into existence...", name);

            let config = prompts::ask(name)?;

            scaffold::run(&config)?;
        }

        None => {
            println!("Run 'zila new <project-name>' to get started");
        }
    }

    Ok(())
}
