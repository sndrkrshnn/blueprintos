use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub const DEFAULT_BUFFER_SIZE: usize = 64;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

pub struct MessageBus {
    topics: Arc<RwLock<HashMap<Topic, broadcast::Sender<Message>>>>,
    agents: Arc<RwLock<Vec<AgentId>>>,
}

impl MessageBus {
    pub async fn new() -> Result<Self> {
        let bus = Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            agents: Arc::new(RwLock::new(Vec::new())),
        };

        for topic in [
            Topic::System,
            Topic::Voice,
            Topic::Files,
            Topic::Network,
            Topic::Shell,
            Topic::Calendar,
            Topic::UI,
            Topic::Error,
        ] {
            bus.ensure_topic(topic).await;
        }

        Ok(bus)
    }

    async fn ensure_topic(&self, topic: Topic) {
        let mut map = self.topics.write().await;
        map.entry(topic).or_insert_with(|| {
            let (tx, _) = broadcast::channel(DEFAULT_BUFFER_SIZE);
            tx
        });
    }

    pub async fn subscribe(&self, agent: AgentId, topic: Topic) -> broadcast::Receiver<Message> {
        self.ensure_topic(topic.clone()).await;
        self.agents.write().await.push(agent);

        let map = self.topics.read().await;
        map.get(&topic).expect("topic exists").subscribe()
    }

    pub async fn send(&self, topic: Topic, payload: impl Into<serde_json::Value>) -> Result<()> {
        self.ensure_topic(topic.clone()).await;
        let msg = Message {
            id: Uuid::new_v4().to_string(),
            topic: topic.clone(),
            payload: payload.into(),
            sender: AgentId("system".into()),
            timestamp: now_ms(),
        };

        let map = self.topics.read().await;
        if let Some(tx) = map.get(&topic) {
            let _ = tx.send(msg);
        }
        Ok(())
    }

    pub async fn list_agents(&self) -> Result<Vec<String>> {
        let agents = self.agents.read().await;
        Ok(agents.iter().map(|a| a.0.clone()).collect())
    }

    pub async fn start_voice_mode(&self, api_endpoint: String) -> Result<()> {
        tracing::info!("Voice/STS mode active. Endpoint: {}", api_endpoint);
        self.run().await
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    }

    pub async fn run_cli(&self) -> Result<()> {
        use std::io::{self, Write};
        println!("MuninOS CLI mode. Type 'quit' to exit.");

        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            if stdin.read_line(&mut input)? == 0 {
                break;
            }
            let input = input.trim();
            if input.eq_ignore_ascii_case("quit") || input.is_empty() {
                break;
            }
            self.send(Topic::System, input.to_string()).await?;
            println!("sent: {}", input);
        }

        Ok(())
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bus_smoke() {
        let bus = MessageBus::new().await.unwrap();
        let mut rx = bus.subscribe(AgentId("tester".into()), Topic::System).await;
        bus.send(Topic::System, "hello").await.unwrap();
        let msg = rx.recv().await.unwrap();
        assert_eq!(msg.payload, serde_json::Value::String("hello".into()));
    }
}
