use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::{Result, anyhow, Context};
use std::time::Duration;

pub struct AIClient {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

impl AIClient {
     pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("Échec de création du client HTTP"),
            api_key: api_key.to_string(),
        }
    }

    pub async fn analyze_text(&self, text: &str) -> Result<String> {
        let url = "https://api.groq.com/openai/v1/chat/completions";

        let system_prompt = "Tu es un professeur expérimenté. Analyse ce texte, convertis-le en Markdown clair, et si c'est un exercice, fournis une solution détaillée.";
        
        let request = GroqRequest {
            model: "llama-3.3-70b-versatile".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: text.to_string(),
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(4096),
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Échec de l'envoi de la requête à Groq")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            return Err(anyhow!("Erreur API Groq ({}): {}", status, error_body));
        }

        let groq_response: GroqResponse = response.json()
            .await
            .context("Échec du parsing de la réponse JSON de Groq")?;

        let text_result = groq_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("Réponse vide de Groq"))?;

        Ok(text_result)
    }
}

pub async fn analyze_text(text: &str, api_key: &str) -> Result<String> {
    let client = AIClient::new(api_key);
    client.analyze_text(text).await
}
