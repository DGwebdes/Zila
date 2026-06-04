

mod cli;
mod prompts;
mod scaffold;
mod templates;

use cli::{Cli, Commands};

use clap::Parser;

fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            println!("GodZila {} into existence...", name);


            let config = prompts::ask(name)?;

            scaffold::run(&config)?;
    
        }

        None => {
            println!("Run 'zila new <project-name>' to get started");
        }

    }

    Ok(())
}
