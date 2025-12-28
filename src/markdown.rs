use anyhow::{Context, Result};
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Normalise le texte brut (souvent issu d'OCR) en supprimant les caractères bizarres
/// et en uniformisant les sauts de ligne.
pub fn normalize_text(text: &str) -> String {
    // Suppression des caractères de contrôle non désirés
    let cleaned = text.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>();

    // Remplacement des sauts de ligne multiples par un double saut de ligne Max
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    let normalized = re_newlines.replace_all(&cleaned, "\n\n");

    normalized.trim().to_string()
}

/// Convertit un texte brut structuré en Markdown propre.
/// 
/// Détecte :
/// - Les titres (lignes courtes finissant par pas de ponctuation ou commençant par chiffre)
/// - Les listes (commençant par -, *, • ou chiffre.)
/// - Les blocs de code (si indentés ou entre lignes suspectes)
pub fn to_markdown(text: &str) -> String {
    let mut md = String::new();
    let lines: Vec<&str> = text.lines().collect();
    
    let re_list = Regex::new(r"^(\s*[-*•]|\s*\d+\.)\s+").unwrap();
    let re_maybe_header = Regex::new(r"^[A-ZÀ-Z1-9].{1,50}$").unwrap();

    let mut in_code_block = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            md.push('\n');
            continue;
        }

        // Détection de titre simple : ligne courte, pas de point final, et ligne suivante vide ou existante
        if i == 0 || (lines.get(i-1).map_or(true, |l| l.trim().is_empty())) {
            if re_maybe_header.is_match(trimmed) && !trimmed.ends_with('.') && !re_list.is_match(trimmed) {
                md.push_str("## ");
                md.push_str(trimmed);
                md.push('\n');
                continue;
            }
        }

        // Détection de liste
        if re_list.is_match(line) {
            md.push_str(line);
            md.push('\n');
            continue;
        }

        // Par défaut, paragraphe standard
        md.push_str(line);
        md.push('\n');
    }

    md.trim().to_string()
}

/// Sauvegarde le contenu Markdown dans un fichier dans le dossier temp.
/// 
/// # Exemple
/// ```rust
/// use telegram_ai_analyzer::markdown::save_as_markdown_file;
/// 
/// let content = "# Rapport\nContenu...";
/// save_as_markdown_file(content, "rapport_001.md").expect("Erreur de sauvegarde");
/// ```
pub fn save_as_markdown_file(content: &str, filename: &str) -> Result<()> {
    let temp_dir = Path::new("temp");
    if !temp_dir.exists() {
        std::fs::create_dir_all(temp_dir).context("Impossible de créer le dossier temp")?;
    }

    let file_path = temp_dir.join(filename);
    let mut file = File::create(&file_path)
        .with_context(|| format!("Impossible de créer le fichier {:?}", file_path))?;

    file.write_all(content.as_bytes())
        .with_context(|| format!("Erreur d'écriture dans {:?}", file_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        let input = "Texte\n\n\n\nSuite";
        assert_eq!(normalize_text(input), "Texte\n\nSuite");
    }

    #[test]
    fn test_to_markdown_header() {
        let input = "Ceci est un titre\n\nUn paragraphe.";
        let output = to_markdown(input);
        assert!(output.contains("## Ceci est un titre"));
    }
}
