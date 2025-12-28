# 🎯 Telegram AI Analyzer

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Groq AI](https://img.shields.io/badge/AI-Llama--3.3--70B-blue.svg)](https://groq.com)
[![Teloxide](https://img.shields.io/badge/Framework-Teloxide-blue.svg)](https://github.com/teloxide/teloxide)

**Telegram AI Analyzer** is a powerful Telegram bot built with Rust to extract, analyze, and structure the content of your documents (images and PDFs) using artificial intelligence.

---

## ✨ Main Features

- 📸 **Multi-language OCR**: Text extraction from images (JPG, PNG, WebP) via Tesseract OCR.
- 📄 **Hybrid PDF Analysis**: 
    - Direct extraction for text-based PDFs.
    - Automatic OCR for scanned PDFs.
- 🧠 **Artificial Intelligence**: Deep analysis by Llama 3.3 70B (Groq AI).
- 🎓 **Educational Mode**: The bot acts as an experienced teacher to explain concepts and solve exercises.
- 📝 **Markdown Formatting**: Automatic conversion of raw text into clean, structured Markdown documents.
- 📁 **Automatic Reports**: Sends `.md` files for large analyses.
- 🧹 **Clean Management**: Automatic cleanup of temporary files.

---

## 🛠️ System Prerequisites

To function, this project requires a few external tools:

### 1. Rust
Install the latest version of Rust via [rustup.rs](https://rustup.rs/).
```bash
rustc --version # 1.70+ recommended
```

### 2. Tesseract OCR (Essential for Image/OCR)
- **Windows**: Download the installer via [UB Mannheim](https://github.com/UB-Mannheim/tesseract/wiki). Add `tesseract` to your PATH.
- **Ubuntu/Debian**: `sudo apt install tesseract-ocr libtesseract-dev`
- **macOS**: `brew install tesseract`

### 3. Poppler Utils (Essential for PDFs)
Poppler provides `pdftotext` and `pdftoppm`.
- **Windows**: Download binaries via [Poppler for Windows](http://blog.alivate.com.au/poppler-windows/) or via `conda install -c conda-forge poppler`.
- **Ubuntu/Debian**: `sudo apt install poppler-utils`
- **macOS**: `brew install poppler`

---

## 📦 Installation

1. **Clone the project**
   ```bash
   git clone https://github.com/your-repo/telegram-ai-analyzer.git
   cd telegram-ai-analyzer
   ```

2. **Environment Configuration**
   Copy the example file and fill in your keys:
   ```bash
   cp .env.example .env
   ```
   Edit `.env`:
   ```env
   TELEGRAM_BOT_TOKEN=your_bot_father_token
   GROQ_API_KEY=your_groq_api_key
   ```

3. **Compilation**
   ```bash
   cargo build --release
   ```

---

## 🚀 Usage

Run the bot in development mode:
```bash
cargo run
```

### Available Commands
- `/start` - Displays the welcome message and instructions.
- `/help` - Displays help and supported formats.

### Example Flow
1. Send a photo of a math exercise.
2. The bot responds: "📄 Analysis in progress... I'm reading your document.".
3. The bot extracts the text, sends it to the AI, and returns a detailed solution formatted in Markdown.

---

## 🐳 Docker Deployment

The project is container-ready for easy deployment.

### 1. Simple Deployment
Ensure your `.env` is properly configured, then run:
```bash
docker compose up -d
```

### 2. Manual Build
If you want to build the image manually:
```bash
docker compose build
```

### 3. Monitoring
Check logs in real-time:
```bash
docker compose logs -f
```


---

## 🧪 Tests

To run unit tests (especially on the Markdown and PDF modules):
```bash
cargo test
```

---

## 🤝 Contribution

Contributions are welcome! Feel free to open an Issue or a Pull Request to suggest improvements or fix bugs.

---

## 📄 License

This project is licensed under the **MIT** License. See the [LICENSE](LICENSE) file for more details.

---

## 🙏 Acknowledgements

- [Teloxide](https://teloxide.rs/) for the superb bot framework.
- [Groq](https://groq.com/) for the power of fast inference.
- [Poppler](https://poppler.freedesktop.org/) and [Tesseract](https://tesseract-ocr.github.io/) for the extraction tools.
