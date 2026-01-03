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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Hello) => {
            println!("Hello from Draftline 👋");
        }
        None => {
            println!("Hello from Draftline 👋");
        }
    }
}
