use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;

/// Client pour interagir avec l'API OpenAI.
pub struct AIClient {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

impl AIClient {
    /// Crée une nouvelle instance de AIClient.
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("Échec de la création du client HTTP"),
            api_key: api_key.to_string(),
        }
    }

    /// Analyse le texte fourni via l'API OpenAI (GPT-4o-mini).
    /// 
    /// Le système agit comme un professeur expérimenté pour structurer 
    /// le texte et résoudre d'éventuels exercices.
    pub async fn analyze_text(&self, text: &str) -> Result<String> {
        let system_prompt = "Tu es un professeur expérimenté. Analyse ce texte, \
                             convertis-le en Markdown clair, et si c'est un exercice, \
                             fournis une solution détaillée.";

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
            temperature: 0.3,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Échec de l'envoi de la requête à OpenAI")?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(anyhow!("Erreur API OpenAI ({}): {}", response.status(), error_body));
        }

        let chat_response: ChatResponse = response.json()
            .await
            .context("Échec du parsing de la réponse JSON d'OpenAI")?;

        let result = chat_response.choices
            .first()
            .ok_or_else(|| anyhow!("Aucune réponse reçue d'OpenAI"))?
            .message
            .content
            .clone();

        Ok(result)
    }
}

/// Helper pour une utilisation rapide sans instanciation manuelle.
/// 
/// # Exemple
/// ```rust
/// use telegram_ai_analyzer::ai::analyze_text;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let result = analyze_text("2+2=?", "votre_cle_api").await?;
///     println!("{}", result);
///     Ok(())
/// }
/// ```
pub async fn analyze_text(text: &str, api_key: &str) -> Result<String> {
    let client = AIClient::new(api_key);
    client.analyze_text(text).await
}
