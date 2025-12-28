use anyhow::{Context, Result};
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;

 pub fn normalize_text(text: &str) -> String {
    let cleaned = text.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>();
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    let normalized = re_newlines.replace_all(&cleaned, "\n\n");

    normalized.trim().to_string()
}

 
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

        if i == 0 || (lines.get(i-1).map_or(true, |l| l.trim().is_empty())) {
            if re_maybe_header.is_match(trimmed) && !trimmed.ends_with('.') && !re_list.is_match(trimmed) {
                md.push_str("## ");
                md.push_str(trimmed);
                md.push('\n');
                continue;
            }
        }

        if re_list.is_match(line) {
            md.push_str(line);
            md.push('\n');
            continue;
        }

        md.push_str(line);
        md.push('\n');
    }

    md.trim().to_string()
}

pub fn save_as_markdown_file(content: &str, filename: &str) -> Result<()> {
    let temp_dir = Path::new("temp");
    if !temp_dir.exists() {
        std::fs::create_dir_all(temp_dir).context("Impossible de créer le dossier temp")?;
    }

    let file_path = temp_dir.join(filename);
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
