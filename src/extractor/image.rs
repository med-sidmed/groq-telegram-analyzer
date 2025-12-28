use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::path::Path;

pub fn extract_text_from_image(path: &str) -> Result<String> {
    let image_path = Path::new(path);
    if !image_path.exists() {
        return Err(anyhow!("Le fichier image n'existe pas : {}", path));
    }

    // Tentative de lancement de tesseract
    let mut cmd = Command::new("tesseract");
    
    // Sur Windows, on tente des chemins communs si "tesseract" n'est pas dans le PATH
    #[cfg(target_os = "windows")]
    {
        let common_paths = [
            r"C:\Program Files\Tesseract-OCR\tesseract.exe",
            r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
        ];
        
        let in_path = Command::new("where").arg("tesseract").output().is_ok();
        
        if !in_path {
            for p in common_paths {
                if Path::new(p).exists() {
                    cmd = Command::new(p);
                    break;
                }
            }
        }
    }

    let output = cmd
        .arg(path)
        .arg("stdout")
        .arg("-l")
        .arg("fra+eng+ara")
        .output()
        .context("Échec du lancement de Tesseract. Cet outil est nécessaire pour lire les images.\n\n\
                 👉 Téléchargez l'installateur ici : https://github.com/UB-Mannheim/tesseract/wiki\n\
                 👉 Assurez-vous de l'ajouter à votre PATH Windows.")?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Erreur Tesseract (code {}) : {}", output.status.code().unwrap_or(-1), error_msg));
    }

    let raw_text = String::from_utf8(output.stdout)
        .context("Le texte extrait par Tesseract n'est pas de l'UTF-8 valide")?;

    let cleaned_text = clean_extracted_text(&raw_text);

    if cleaned_text.is_empty() {
        return Err(anyhow!("Aucun texte n'a pu être extrait de l'image"));
    }

    Ok(cleaned_text)
}

fn clean_extracted_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
