use dotenvy::dotenv;
use std::env;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialisation de l'environnement et du logger
    dotenv().ok();
    pretty_env_logger::init();

    log::info!("Initialisation de l'Analyseur IA Telegram...");

    // 2. Récupération des clés API
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .context("TELEGRAM_BOT_TOKEN doit être défini dans le fichier .env")?;
    let openai_key = env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY doit être défini dans le fichier .env")?;

    // 3. Lancement du bot
    telegram_ai_analyzer::bot::run(bot_token, openai_key).await?;

    Ok(())
}
