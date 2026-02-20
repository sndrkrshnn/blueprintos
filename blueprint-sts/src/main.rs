//! BlueprintOS Speech-to-Speech Service
//!
//! Real-time audio streaming to/from Qwen3 Omni API
//! Provides low-latency speech-to-speech interaction.

use anyhow::{Context, Result};
use async_stream::stream;
use bytes::Bytes;
use clap::{Parser, Subcommand};
use futures::{Stream, StreamExt, TryStreamExt};
use ringbuf::RingBuffer;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "blueprint-sts")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Qwen3 Omni API endpoint
    #[arg(long, default_value = "https://api.qwen.ai/v1/omni/chat")]
    api_endpoint: String,

    /// API key for Qwen3 Omni
    #[arg(long, env = "QWEN_API_KEY")]
    api_key: Option<String>,

    /// Audio sample rate
    #[arg(long, default_value = "16000")]
    sample_rate: u32,

    /// Audio channels (1 = mono)
    #[arg(long, default_value = "1")]
    channels: u16,

    /// Chunk size in ms for streaming
    #[arg(long, default_value = "100")]
    chunk_ms: u64,

    /// Enable wake word detection
    #[arg(long, default_value = "true")]
    wake_word: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start Speech-to-Speech service
    Start,
    /// Test audio input/output
    TestAudio,
    /// Send audio file and get response
    Interact { audio_file: String },
}

/// Audio chunk for streaming
#[derive(Debug, Clone)]
pub struct AudioChunk {
    pub data: Vec<f32>,
    pub timestamp: u64,
    pub is_final: bool,
}

/// STS Session - manages a single conversation
pub struct STSSession {
    id: String,
    audio_buffer: Arc<RwLock<Vec<f32>>>,
    response_buffer: Arc<RwLock<Vec<f32>>>,
    is_active: bool,
}

impl STSSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            audio_buffer: Arc::new(RwLock::new(Vec::new())),
            response_buffer: Arc::new(RwLock::new(Vec::new())),
            is_active: false,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub async fn add_input(&self, chunk: AudioChunk) {
        let mut buffer = self.audio_buffer.write().await;
        buffer.extend(chunk.data);
    }

    pub async fn get_audio(&self) -> Vec<f32> {
        let buffer = self.audio_buffer.read().await;
        buffer.clone()
    }

    pub async fn set_response(&self, audio: Vec<f32>) {
        let mut buffer = self.response_buffer.write().await;
        *buffer = audio;
    }

    pub async fn take_response(&self) -> Vec<f32> {
        let mut buffer = self.response_buffer.write().await;
        std::mem::take(&mut *buffer)
    }
}

/// Qwen3 Omni API client
pub struct QwenOmniClient {
    endpoint: String,
    api_key: String,
    client: reqwest::Client,
}

impl QwenOmniClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            endpoint,
            api_key,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .context("Failed to build HTTP client")?,
        }
    }

    /// Send audio and receive streaming response
    pub fn streaming_chat(
        &self,
        audio_chunks: impl Stream<Item = Result<Bytes>> + Unpin,
    ) -> impl Stream<Item = Result<Bytes>> {
        let endpoint = self.endpoint.clone();
        let api_key = self.api_key.clone();

        stream! {
            let mut multipart = reqwest::multipart::Form::new();

            // Add the audio stream as a part
            let part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(
                audio_chunks.map_ok(|b| bytes::BytesMut::from(&b[..]).freeze())
            ))
            .file_name("audio.wav")
            .mime_str("audio/wav")?;

            multipart = multipart.part("audio", part);

            // Add parameters
            let params = serde_json::json!({
                "model": "qwen3-omni",
                "stream": true,
                "max_tokens": 1000,
            });
            multipart = multipart.text("payload", params.to_string());

            let response = self.client
                .post(&endpoint)
                .header("Authorization", format!("Bearer {}", api_key))
                .multipart(multipart)
                .send()
                .await
                .context("Failed to send request")?;

            let mut stream = response.bytes_stream();

            while let Some(chunk) = stream.next().await {
                yield chunk.context("Failed to read response chunk");
            }
        }
    }

    /// Simple non-streaming chat for testing
    pub async fn chat(&self, audio_data: &[u8]) -> Result<Vec<u8>> {
        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .multipart(form)
            .send()
            .await?;

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

/// Audio I/O handler
pub struct AudioIO {
    sample_rate: u32,
    channels: u16,
    chunk_size: usize,
}

impl AudioIO {
    pub fn new(sample_rate: u32, channels: u16, chunk_ms: u64) -> Self {
        Self {
            sample_rate,
            channels,
            chunk_size: (sample_rate as usize * channels as usize * chunk_ms as usize / 1000),
        }
    }

    /// Capture audio from microphone
    pub fn capture_stream(&self) -> impl Stream<Item = AudioChunk> + Unpin {
        // This is a placeholder - actual implementation uses cpal
        stream! {
            // In real implementation:
            // - Open microphone stream with cpal
            // - Read frames in chunks
            // - Yield AudioChunk for each read

            // Placeholder loop
            loop {
                // Would yield actual audio data here
                yield AudioChunk {
                    data: vec![0.0; self.chunk_size],
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    is_final: false,
                };
                sleep(Duration::from_millis(self.chunk_ms)).await;
            }
        }
    }

    /// Play audio to speakers
    pub async fn play(&self, audio_data: &[f32]) {
        // In real implementation:
        // - Open speaker stream with cpal
        // - Write audio frames
        // - Wait for playback to complete
    }
}

/// Simple wake word detector (placeholder)
pub struct WakeWordDetector {
    wake_word: String,
    threshold: f32,
}

impl WakeWordDetector {
    pub fn new(wake_word: &str) -> Self {
        Self {
            wake_word: wake_word.to_lowercase(),
            threshold: 0.7,
        }
    }

    pub async fn detect(&self, audio: &[f32]) -> bool {
        // Placeholder - real implementation would use:
        // - TinyML model (like Porcupine, or custom CNN)
        // - Or streaming keyword spotting

        // For now, just log
        tracing::debug!("Wake word check - audio length: {}", audio.len());
        false
    }
}

/// Main STS Service
pub struct STSService {
    audio_io: AudioIO,
    client: QwenOmniClient,
    wake_word: WakeWordDetector,
    session: Arc<RwLock<STSSession>>,
}

impl STSService {
    pub fn new(args: &Args) -> Result<Self> {
        let audio_io = AudioIO::new(
            args.sample_rate,
            args.channels,
            args.chunk_ms,
        );

        let api_key = args.api_key.clone()
            .or_else(|| std::env::var("QWEN_API_KEY").ok())
            .context("Qwen API key required. Set --api-key or QWEN_API_KEY env var")?;

        let client = QwenOmniClient::new(
            args.api_endpoint.clone(),
            api_key,
        );

        let wake_word = WakeWordDetector::new("hey blueprint");

        Ok(Self {
            audio_io,
            client,
            wake_word,
            session: Arc::new(RwLock::new(STSSession::new())),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Starting BlueprintOS Speech-to-Speech service...");

        let audio_stream = self.audio_io.capture_stream();

        futures::pin_mut!(audio_stream);

        loop {
            while let Some(chunk) = audio_stream.next().await {
                // Check for wake word
                if self.wake_word.detect(&chunk.data).await {
                    tracing::info!("Wake word detected!");
                    self.handle_conversation().await?;
                }
            }
        }
    }

    async fn handle_conversation(&mut self) -> Result<()> {
        let mut session = self.session.write().await;
        session.start();

        tracing::info!("Session {} started", session.id);

        // In real implementation:
        // 1. Collect user audio until silence/end
        // 2. Send to Qwen3 Omni
        // 3. Stream response audio
        // 4. Play response

        // Simulate conversation
        tokio::time::sleep(Duration::from_secs(5)).await;

        session.stop();
        tracing::info!("Session {} ended", session.id);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Commands::Start => {
            let mut service = STSService::new(&args)?;
            service.run().await?;
        }
        Commands::TestAudio => {
            println!("Audio test - would verify microphone/speaker");
        }
        Commands::Interact { audio_file } => {
            println!("Would process file: {}", audio_file);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_creation() {
        let session = STSSession::new();
        assert!(!session.id.is_empty());
    }

    #[tokio::test]
    async fn test_wake_word_detector() {
        let detector = WakeWordDetector::new("hey blueprint");
        let result = detector.detect(&[0.0; 16000]).await;
        assert!(!result);
    }
}
