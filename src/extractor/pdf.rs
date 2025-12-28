use anyhow::{anyhow, Context, Result};
use std::process::Command;
use std::path::{Path, PathBuf};
use lopdf::Document;
use std::fs;

 
pub fn extract_text_from_pdf(path: &str) -> Result<String> {
    let pdf_path = Path::new(path);
    if !pdf_path.exists() {
        return Err(anyhow!("Le fichier PDF n'existe pas : {}", path));
    }

    if is_scanned_pdf(path)? {
        extract_scanned_text(path)
    } else {
        extract_searchable_text(path)
    }
}

fn is_scanned_pdf(path: &str) -> Result<bool> {
    let doc = Document::load(path)
        .map_err(|e| anyhow!("Erreur lors du chargement du PDF avec lopdf: {}", e))?;
    
    let mut text_found = false;
    let pages = doc.get_pages();
    
    for (page_num, _) in pages.iter().take(3) {
        let text = doc.extract_text(&[*page_num])
            .map_err(|e| anyhow!("Erreur d'extraction sur la page {}: {}", page_num, e))?;
        
        if text.trim().len() > 10 {
            text_found = true;
            break;
        }
    }

    Ok(!text_found)
}

fn extract_searchable_text(path: &str) -> Result<String> {
    let mut cmd = Command::new("pdftotext");

    #[cfg(target_os = "windows")]
    {
        let common_paths = [
            r"C:\Program Files\poppler-23.08.0\Library\bin\pdftotext.exe",
            r"C:\poppler\bin\pdftotext.exe",
        ];
        if Command::new("where").arg("pdftotext").output().is_err() {
            for p in common_paths {
                if Path::new(p).exists() {
                    cmd = Command::new(p);
                    break;
                }
            }
        }
    }

    let output = cmd
        .arg("-layout")
        .arg(path)
        .arg("-")
        .output()
        .context("Échec de l'exécution de 'pdftotext'. Poppler est nécessaire pour lire les PDF.\n\n\
                 👉 Guide Windows : http://blog.alivate.com.au/poppler-windows/")?;

    if !output.status.success() {
        return Err(anyhow!("Erreur pdftotext : {}", String::from_utf8_lossy(&output.stderr)));
    }

    let text = String::from_utf8(output.stdout)
        .context("Le texte extrait par pdftotext n'est pas de l'UTF-8 valide")?;

    Ok(clean_pdf_text(&text))
}

fn extract_scanned_text(path: &str) -> Result<String> {
    let temp_dir = Path::new("temp").join("pdf_ocr");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
    }

    let file_stem = Path::new(path).file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("doc");

    let output = Command::new("pdftoppm")
        .arg("-png")
        .arg("-r")
        .arg("300")
        .arg(path)
        .arg(temp_dir.join(file_stem))
        .output()
        .context("Échec de 'pdftoppm'. Est-il installé (poppler-utils) ?")?;

    if !output.status.success() {
        return Err(anyhow!("Erreur pdftoppm : {}", String::from_utf8_lossy(&output.stderr)));
    }

     let mut full_text = String::new();
    
    let mut entries: Vec<PathBuf> = fs::read_dir(&temp_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "png") && p.to_str().map_or(false, |s| s.contains(file_stem)))
        .collect();
    
    entries.sort();

    for img_path in entries {
        let page_text = super::image::extract_text_from_image(img_path.to_str().unwrap())?;
        full_text.push_str(&page_text);
        full_text.push_str("\n\n");
        
        let _ = fs::remove_file(img_path);
    }

    Ok(clean_pdf_text(&full_text))
}

fn clean_pdf_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_pdf_text() {
        let input = "  Ligne 1  \n\n  Ligne 2  ";
        assert_eq!(clean_pdf_text(input), "Ligne 1\nLigne 2");
    }
}
