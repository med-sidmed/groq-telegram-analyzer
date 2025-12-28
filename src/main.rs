use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    setup().context("Échec de l'initialisation du système")?;
    let (bot_token, groq_key) = validate_env().context("Erreur de configuration de l'environnement")?;
    log::info!("🚀 Démarrage de l'Analyseur IA Telegram (Groq)...");
    log::info!("Système prêt. En attente de messages...");
    if let Err(e) = telegram_ai_analyzer::bot::run(bot_token, groq_key).await {
        log::error!("Le bot s'est arrêté de manière inattendue : {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn setup() -> Result<()> {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,telegram_ai_analyzer=info");
    }
    pretty_env_logger::init();
    let temp_dir = Path::new("temp");
    if !temp_dir.exists() {
        fs::create_dir_all(temp_dir)
            .context("Impossible de créer le dossier 'temp/'")?;
        log::debug!("Dossier 'temp/' créé avec succès.");
    }

    Ok(())
}

fn validate_env() -> Result<(String, String)> {
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| anyhow::anyhow!(
            "Variable 'TELEGRAM_BOT_TOKEN' manquante.\n\
             👉 Créez un fichier '.env' basé sur '.env.example' et ajoutez votre token."
        ))?;

    let groq_key = env::var("GROQ_API_KEY")
        .map_err(|_| anyhow::anyhow!(
            "Variable 'GROQ_API_KEY' manquante.\n\
             👉 Ajoutez votre clé API Groq dans le fichier '.env'."
        ))?;

    Ok((bot_token, groq_key))
}
