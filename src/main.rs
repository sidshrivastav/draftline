use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "draftline")]
#[command(version)]
#[command(about = "Draftline - a note-driven CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print hello message
    Hello,
    Index,
    Expand
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hello) => {
            println!("Hello from Draftline 👋");
        }
        Some(Commands::Index) => {
            println!("Ran index!");
        }
        Some(Commands::Expand) => {
            println!("Ran expand!");
        }
        None => {
            println!("Hello from Draftline 👋");
        }
    }
}
