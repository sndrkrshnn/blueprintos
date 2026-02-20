use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "blueprint-sts")]
#[command(author, version, about = "BlueprintOS Speech-to-Speech service")]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = "https://api.qwen.ai/v1/omni/chat")]
    api_endpoint: String,

    #[arg(long, env = "QWEN_API_KEY")]
    api_key: Option<String>,

    #[arg(long, default_value_t = 16000)]
    sample_rate: u32,

    #[arg(long, default_value_t = 1)]
    channels: u16,

    #[arg(long, default_value_t = 100)]
    chunk_ms: u64,

    #[arg(long, default_value_t = true)]
    wake_word: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start,
    TestAudio,
    Interact { audio_file: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct QwenRequest {
    model: String,
    mode: String,
}

struct STSService {
    session_id: String,
    api_endpoint: String,
    api_key: String,
    client: Client,
}

impl STSService {
    fn new(args: &Args) -> Result<Self> {
        let api_key = args
            .api_key
            .clone()
            .or_else(|| std::env::var("QWEN_API_KEY").ok())
            .context("QWEN_API_KEY is required")?;

        Ok(Self {
            session_id: Uuid::new_v4().to_string(),
            api_endpoint: args.api_endpoint.clone(),
            api_key,
            client: Client::new(),
        })
    }

    async fn run(&self) -> Result<()> {
        info!("Starting BlueprintOS STS session: {}", self.session_id);
        info!("Pipeline: audio-in -> qwen3-omni -> audio-out");

        loop {
            // Placeholder runtime loop for now.
            // Next step: wire microphone capture + websocket/http streaming.
            sleep(Duration::from_secs(2)).await;
            info!("STS service alive");
        }
    }

    async fn ping_provider(&self) -> Result<()> {
        let payload = QwenRequest {
            model: "qwen3-omni".to_string(),
            mode: "speech_to_speech".to_string(),
        };

        let resp = self
            .client
            .post(&self.api_endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match resp {
            Ok(r) => {
                info!("Provider reachable: HTTP {}", r.status());
                Ok(())
            }
            Err(e) => {
                error!("Provider ping failed: {}", e);
                Err(e.into())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    match args.command {
        Commands::Start => {
            let service = STSService::new(&args)?;
            // best-effort connectivity check
            let _ = service.ping_provider().await;
            service.run().await?;
        }
        Commands::TestAudio => {
            info!("Audio test stub (TODO): mic/speaker verification");
        }
        Commands::Interact { audio_file } => {
            info!("Interact stub (TODO): processing file {}", audio_file);
        }
    }

    Ok(())
}
