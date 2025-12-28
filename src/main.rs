use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::Path;

/// Point d'entrée principal de l'Analyseur IA Telegram.
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialisation de l'environnement (Logger, Dossiers, etc.)
    setup().context("Échec de l'initialisation du système")?;

    // 2. Validation des variables d'environnement
    let (bot_token, openai_key) = validate_env().context("Erreur de configuration de l'environnement")?;

    log::info!("🚀 Démarrage de l'Analyseur IA Telegram...");
    log::info!("Système prêt. En attente de messages...");

    // 3. Lancement du bot (boucle infinie avec gestion Ctrl+C)
    if let Err(e) = telegram_ai_analyzer::bot::run(bot_token, openai_key).await {
        log::error!("Le bot s'est arrêté de manière inattendue : {}", e);
        std::process::exit(1);
    }

    Ok(())
}

/// Configure les éléments de base : logging et structure de dossiers.
fn setup() -> Result<()> {
    // Initialisation du logger avec un format lisible
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,telegram_ai_analyzer=info");
    }
    pretty_env_logger::init();

    // Création du dossier temporaire pour les fichiers
    let temp_dir = Path::new("temp");
    if !temp_dir.exists() {
        fs::create_dir_all(temp_dir)
            .context("Impossible de créer le dossier 'temp/'")?;
        log::debug!("Dossier 'temp/' créé avec succès.");
    }

    Ok(())
}

/// Vérifie et récupère les clés API nécessaires.
/// Retourne (TELEGRAM_BOT_TOKEN, OPENAI_API_KEY).
fn validate_env() -> Result<(String, String)> {
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| anyhow::anyhow!(
            "Variable 'TELEGRAM_BOT_TOKEN' manquante.\n\
             👉 Créez un fichier '.env' basé sur '.env.example' et ajoutez votre token."
        ))?;

    let openai_key = env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow::anyhow!(
            "Variable 'OPENAI_API_KEY' manquante.\n\
             👉 Ajoutez votre clé API OpenAI dans le fichier '.env'."
        ))?;

    Ok((bot_token, openai_key))
}
