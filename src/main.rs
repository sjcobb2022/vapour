use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init,
    /// Create a new resource
    Make,
    /// Run the project
    Run,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing a new project...");
            // Here you can add your initialization logic
            // unimplemented!();
        }
        Commands::Make => {
            println!("Creating a new resource...");
            // Here you can add your make logic
            // unimplemented!();
        }
        Commands::Run => {
            println!("Running the project...");
            // Here you can add your run logic
            // unimplemented!();
        }
    }
}
