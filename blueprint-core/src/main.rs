use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber;

mod bus;

use bus::MessageBus;

#[derive(Parser, Debug)]
#[command(name = "blueprint-core")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Enable speech mode (STS integration)
    #[arg(long, default_value_t = false)]
    sts: bool,

    /// Qwen3 Omni API endpoint
    #[arg(long, default_value = "https://api.qwen.ai/v1/omni")]
    api_endpoint: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the MuninOS agent core
    Start,
    /// Send a command to the bus
    Send { message: String },
    /// List registered agents
    ListAgents,
    /// Run in CLI mode (no voice)
    Cli,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let bus = MessageBus::new().await?;

    match args.command {
        Commands::Start => {
            tracing::info!("Starting MuninOS Core with sts={}", args.sts);
            if args.sts {
                bus.start_voice_mode(args.api_endpoint).await?;
            } else {
                bus.run().await?;
            }
        }
        Commands::Send { message } => {
            bus.send(bus::Topic::System, message).await?;
        }
        Commands::ListAgents => {
            let agents = bus.list_agents().await?;
            tracing::info!("Registered agents: {:?}", agents);
        }
        Commands::Cli => {
            bus.run_cli().await?;
        }
    }

    Ok(())
}
