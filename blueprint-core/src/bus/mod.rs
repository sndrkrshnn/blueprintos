use anyhow::Result;
use async_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::wrappers::ReceiverStream;
use futures::StreamExt;

pub const DEFAULT_BUFFER_SIZE: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub topic: Topic,
    pub payload: serde_json::Value,
    pub sender: AgentId,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topic {
    System,
    Voice,
    Files,
    Network,
    Shell,
    Calendar,
    UI,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentId(pub String);

pub struct MessageBus {
    topics: Arc<RwLock<HashMap<Topic, Sender<Message>>>>,
    agents: Arc<RwLock<HashMap<AgentId, Sender<Message>>>>,
}

impl MessageBus {
    pub async fn new() -> Result<Self> {
        let topics = Arc::new(RwLock::new(HashMap::new()));
        let agents = Arc::new(RwLock::new(HashMap::new()));

        // Initialize default topics
        Self::init_topic(&topics, Topic::System).await;
        Self::init_topic(&topics, Topic::Voice).await;
        Self::init_topic(&topics, Topic::Files).await;
        Self::init_topic(&topics, Topic::Network).await;
        Self::init_topic(&topics, Topic::Shell).await;
        Self::init_topic(&topics, Topic::Calendar).await;
        Self::init_topic(&topics, Topic::UI).await;
        Self::init_topic(&topics, Topic::Error).await;

        Ok(Self { topics, agents })
    }

    async fn init_topic(topics: &Arc<RwLock<HashMap<Topic, Sender<Message>>>>, topic: Topic) {
        let (tx, _rx) = bounded::<Message>(DEFAULT_BUFFER_SIZE);
        let mut map = topics.write().await;
        map.insert(topic, tx);
    }

    pub async fn subscribe(&self, agent: AgentId, topic: Topic) -> Receiver<Message> {
        let topic_map = self.topics.read().await;
        if let Some(tx) = topic_map.get(&topic) {
            let rx = tx.subscribe();
            self.agents.write().await.insert(agent, tx.clone());
            return rx;
        }
        // If topic doesn't exist, create it
        drop(topic_map);
        let (tx, rx) = bounded::<Message>(DEFAULT_BUFFER_SIZE);
        self.topics.write().await.insert(topic, tx.clone());
        self.agents.write().await.insert(agent, tx.clone());
        rx
    }

    pub async fn send(&self, topic: Topic, payload: impl Into<serde_json::Value>) -> Result<()> {
        let topic_map = self.topics.read().await;
        if let Some(tx) = topic_map.get(&topic) {
            let message = Message {
                id: uuid::Uuid::new_v4().to_string(),
                topic: topic.clone(),
                payload: payload.into(),
                sender: AgentId("system".to_string()),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_millis() as u64,
            };
            tx.send(message).await?;
        }
        Ok(())
    }

    pub async fn broadcast(&self, topic: Topic, payload: impl Into<serde_json::Value>) -> Result<()> {
        let message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            topic: topic.clone(),
            payload: payload.into(),
            sender: AgentId("system".to_string()),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_millis() as u64,
        };
        let topic_map = self.topics.read().await;
        for tx in topic_map.values() {
            tx.send(message.clone()).await?;
        }
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        // Main loop - keeps the bus alive
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }

    pub async fn list_agents(&self) -> Result<Vec<String>> {
        let agents = self.agents.read().await;
        Ok(agents.keys().map(|a| a.0.clone()).collect())
    }

    pub async fn start_voice_mode(&self, api_endpoint: String) -> Result<()> {
        // Subscribe to voice topic
        let rx = self.subscribe(AgentId("voice-core".to_string()), Topic::Voice).await;
        let stream = ReceiverStream::new(rx);
        let mut typed_stream = stream.map(|msg| msg.payload);

        // Placeholder for Qwen3 Omni streaming integration
        tracing::info!("Voice mode active, listening on topic: {:?}", Topic::Voice);

        // Keep alive
        self.run().await?;
        Ok(())
    }

    pub async fn run_cli(&self) -> Result<()> {
        println!("BlueprintOS CLI Mode - Type 'quit' to exit");
        use std::io::{self, Write};

        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            if stdin.read_line(&mut input)? == 0 {
                break;
            }

            let input = input.trim().to_string();
            if input.is_empty() || input == "quit" {
                break;
            }

            self.send(Topic::System, input.clone()).await?;
            println!("Sent to system: {}", input);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bus_creation() {
        let bus = MessageBus::new().await.unwrap();
        let agents = bus.list_agents().await.unwrap();
        assert!(agents.is_empty());
    }

    #[tokio::test]
    async fn test_message_send() {
        let bus = MessageBus::new().await.unwrap();
        let rx = bus.subscribe(AgentId("test-agent".to_string()), Topic::System).await;

        bus.send(Topic::System, "test message").await.unwrap();

        let received = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv()).await;
        assert!(received.is_ok());
    }
}
