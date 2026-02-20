use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "bpkg")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List installed packages
    List,
    /// Install a package
    Install { name: String },
    /// Remove a package
    Remove { name: String },
    /// Search for packages
    Search { query: String },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::List => println!("Listing installed packages..."),
        Commands::Install { name } => println!("Installing {}...", name),
        Commands::Remove { name } => println!("Removing {}...", name),
        Commands::Search { query } => println!("Searching for {}...", query),
    }
}
