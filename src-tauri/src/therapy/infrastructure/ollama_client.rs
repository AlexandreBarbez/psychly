use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;

const DEFAULT_OLLAMA_HOST: &str = "http://localhost:11434";

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    base_url: String,
    pub model: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    message: Option<ChatMessageContent>,
    done: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct ChatMessageContent {
    content: String,
}

#[derive(Deserialize, Debug)]
struct GenerateResponse {
    response: Option<String>,
    #[allow(dead_code)]
    done: Option<bool>,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    system: Option<String>,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        let base_url = std::env::var("OLLAMA_HOST")
            .unwrap_or_else(|_| DEFAULT_OLLAMA_HOST.to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url,
            model,
        }
    }

    /// Checks if Ollama is running and reachable.
    pub async fn health_check(&self) -> Result<bool, String> {
        let url = format!("{}/api/tags", self.base_url);
        match self.client.get(&url).timeout(Duration::from_secs(5)).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Sends a chat completion request (non-streaming) and returns the full response.
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<String, String> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            stream: false,
        };

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Ollama error {status}: {body}"));
        }

        let chat_resp: ChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Ollama response: {e}"))?;

        chat_resp
            .message
            .map(|m| m.content)
            .ok_or_else(|| "Empty response from Ollama".to_string())
    }

    /// Sends a chat completion request with streaming.
    /// Returns a channel receiver that yields content chunks.
    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<mpsc::Receiver<Result<String, String>>, String> {
        let url = format!("{}/api/chat", self.base_url);
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            stream: true,
        };

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Ollama stream request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Ollama error {status}: {body}"));
        }

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            use tokio_stream::StreamExt;

            let mut stream = resp.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));

                        // Process complete JSON lines
                        while let Some(newline_pos) = buffer.find('\n') {
                            let line = buffer[..newline_pos].trim().to_string();
                            buffer = buffer[newline_pos + 1..].to_string();

                            if line.is_empty() {
                                continue;
                            }

                            if let Ok(resp) = serde_json::from_str::<ChatResponse>(&line) {
                                if let Some(msg) = resp.message {
                                    if !msg.content.is_empty() {
                                        let _ = tx.send(Ok(msg.content)).await;
                                    }
                                }
                                if resp.done == Some(true) {
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Stream error: {e}"))).await;
                        return;
                    }
                }
            }
        });

        Ok(rx)
    }

    /// Sends a generate request (non-chat, for analysis tasks).
    pub async fn generate(
        &self,
        prompt: String,
        system: Option<String>,
    ) -> Result<String, String> {
        let url = format!("{}/api/generate", self.base_url);
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            system,
        };

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Ollama generate request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Ollama generate error {status}: {body}"));
        }

        let gen_resp: GenerateResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Ollama generate response: {e}"))?;

        gen_resp
            .response
            .ok_or_else(|| "Empty generate response from Ollama".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = OllamaClient::new("qwen2.5:14b".to_string());
        assert_eq!(client.model, "qwen2.5:14b");
        assert!(client.base_url.contains("localhost"));
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "Bonjour".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Bonjour"));
    }

    #[tokio::test]
    async fn test_health_check_unreachable() {
        // Use a port that should not be running Ollama
        let mut client = OllamaClient::new("test".to_string());
        client.base_url = "http://localhost:19999".to_string();
        let result = client.health_check().await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_chat_unreachable() {
        let mut client = OllamaClient::new("test".to_string());
        client.base_url = "http://localhost:19999".to_string();
        let result = client.chat(vec![ChatMessage {
            role: "user".to_string(),
            content: "test".to_string(),
        }]).await;
        assert!(result.is_err());
    }
}
