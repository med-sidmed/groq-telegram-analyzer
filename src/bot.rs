use teloxide::{prelude::*, types::{InputFile}, net::Download};
use teloxide::utils::command::BotCommands;
use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::path::Path;
use tokio::fs;

use crate::extractor::{extract_text_from_image, extract_text_from_pdf};
use crate::ai::AIClient;
use crate::markdown;

/// État partagé du bot.
#[derive(Clone)]
pub struct BotHandler {
    pub openai_key: String,
    pub temp_dir: String,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Commandes supportées :")]
enum Command {
    #[command(description = "Affiche ce message d'aide")]
    Help,
    #[command(description = "Démarre le bot et affiche le message de bienvenue")]
    Start,
}

/// Démarre le bot Telegram.
pub async fn run(token: String, openai_key: String) -> Result<()> {
    let bot = Bot::new(token);
    
    let handler_state = Arc::new(BotHandler {
        openai_key,
        temp_dir: "temp".to_string(),
    });

    // S'assurer que le dossier temp existe
    if !Path::new("temp").exists() {
        fs::create_dir_all("temp").await?;
    }

    log::info!("Démarrage du bot Telegram...");

    let handler = dptree::entry()
        .branch(Update::filter_message()
            .branch(dptree::entry().filter_command::<Command>().endpoint(handle_commands))
            .branch(dptree::entry().endpoint(handle_file_message))
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![handler_state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn handle_commands(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, 
                "👋 Bienvenue sur l'Analyseur IA Telegram !\n\n\
                Envoyez-moi une image ou un document PDF, et je les analyserai pour vous.\n\
                Je peux extraire le texte, le formater proprement et même résoudre vos exercices !")
                .await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
    }
    Ok(())
}

async fn handle_file_message(
    bot: Bot,
    msg: Message,
    state: Arc<BotHandler>,
) -> ResponseResult<()> {
    // 1. Identifier le fichier (Photo ou Document)
    let file_id = if let Some(photo) = msg.photo() {
        // Prendre la meilleure qualité (dernière de la liste)
        photo.last().map(|p| &p.file.id)
    } else if let Some(doc) = msg.document() {
        // Limiter à 10MB
        if doc.file.size > 10 * 1024 * 1024 {
            bot.send_message(msg.chat.id, "⚠️ Le fichier est trop volumineux (max 10MB).").await?;
            return Ok(());
        }
        Some(&doc.file.id)
    } else {
        None
    };

    let file_id = match file_id {
        Some(id) => id,
        None => return Ok(()), // Pas de fichier intéressant
    };

    // 2. Message de progression
    let progress_msg = bot.send_message(msg.chat.id, "📄 Analyse en cours... Je lis votre document.").await?;

    // 3. Téléchargement
    let file = bot.get_file(file_id).await?;
    let extension = msg.document()
        .and_then(|d| d.file_name.as_ref())
        .and_then(|n| Path::new(n).extension())
        .and_then(|e| e.to_str())
        .unwrap_or(if msg.photo().is_some() { "jpg" } else { "" });

    let local_path = format!("{}/{}.{}", state.temp_dir, file_id, extension);
    let mut output_file = fs::File::create(&local_path).await.map_err(|e| anyhow!(e)).unwrap(); // Simplified for now but should be handled better
    
    // Download logic
    if let Err(e) = bot.download_file(&file.path, &mut output_file).await {
         bot.send_message(msg.chat.id, format!("❌ Erreur de téléchargement : {}", e)).await?;
         return Ok(());
    }

    // 4. Extraction selon le type
    let extraction_result = match extension.to_lowercase().as_str() {
        "pdf" => extract_text_from_pdf(&local_path),
        "jpg" | "jpeg" | "png" | "webp" => extract_text_from_image(&local_path),
        _ => Err(anyhow!("Format non supporté ({})", extension)),
    };

    let raw_text = match extraction_result {
        Ok(text) => text,
        Err(e) => {
            let _ = bot.delete_message(msg.chat.id, progress_msg.id).await;
            bot.send_message(msg.chat.id, format!("❌ Erreur d'extraction : {}", e)).await?;
            let _ = fs::remove_file(&local_path).await;
            return Ok(());
        }
    };

    // 5. Analyse IA
    let _ = bot.edit_message_text(msg.chat.id, progress_msg.id, "🧠 Réflexion en cours... L'IA analyse le contenu.").await;
    
    let ai_client = AIClient::new(&state.openai_key);
    let analysis = match ai_client.analyze_text(&raw_text).await {
        Ok(res) => res,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Erreur d'analyse IA : {}", e)).await?;
            let _ = fs::remove_file(&local_path).await;
            return Ok(());
        }
    };

    // 6. Formatage Markdown et réponse
    let final_md = markdown::normalize_text(&analysis);
    
    // Si la réponse est trop longue pour un message Telegram (4096 chars), envoyer en fichier
    if final_md.len() > 4000 {
        let report_name = format!("analyse_{}.md", file_id);
        let _ = markdown::save_as_markdown_file(&final_md, &report_name);
        let report_path = format!("{}/{}", state.temp_dir, report_name);
        
        bot.send_document(msg.chat.id, InputFile::file(&report_path))
            .caption("✅ Analyse terminée ! Le rapport était trop long pour un message direct.")
            .await?;
        
        let _ = fs::remove_file(report_path).await;
    } else {
        bot.send_message(msg.chat.id, final_md).await?;
    }

    // 7. Nettoyage
    let _ = bot.delete_message(msg.chat.id, progress_msg.id).await;
    let _ = fs::remove_file(&local_path).await;

    Ok(())
}
