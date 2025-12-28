use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::path::Path;

/// Extrait le texte d'une image en utilisant Tesseract OCR.
/// 
/// Supporte les langues : français (fra), anglais (eng), arabe (ara).
/// 
/// # Arguments
/// * `path` - Le chemin vers le fichier image (ex: "temp/capture.png")
/// 
/// # Retourne
/// * `Result<String>` - Le texte extrait et nettoyé, ou une erreur si le processus échoue.
/// 
/// # Exemple
/// ```rust
/// use telegram_ai_analyzer::extractor::image::extract_text_from_image;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let text = extract_text_from_image("temp/image.jpg").await?;
///     println!("Texte extrait : {}", text);
///     Ok(())
/// }
/// ```
pub fn extract_text_from_image(path: &str) -> Result<String> {
    // 1. Vérifier si le fichier existe
    let image_path = Path::new(path);
    if !image_path.exists() {
        return Err(anyhow!("Le fichier image n'existe pas : {}", path));
    }

    // 2. Exécuter Tesseract
    // -l fra+eng+ara : spécifie les langues
    // stdout : envoie le résultat vers la sortie standard au lieu d'un fichier
    let output = Command::new("tesseract")
        .arg(path)
        .arg("stdout")
        .arg("-l")
        .arg("fra+eng+ara")
        .output()
        .context("Échec du lancement de Tesseract. Vérifiez s'il est installé et présent dans le PATH.")?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Erreur Tesseract (code {}) : {}", output.status.code().unwrap_or(-1), error_msg));
    }

    // 3. Traiter le texte
    let raw_text = String::from_utf8(output.stdout)
        .context("Le texte extrait par Tesseract n'est pas de l'UTF-8 valide")?;

    let cleaned_text = clean_extracted_text(&raw_text);

    if cleaned_text.is_empty() {
        return Err(anyhow!("Aucun texte n'a pu être extrait de l'image"));
    }

    Ok(cleaned_text)
}

/// Nettoie le texte extrait : suppression des espaces inutiles et des lignes vides.
fn clean_extracted_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
