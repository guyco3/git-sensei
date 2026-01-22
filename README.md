# 👺 GitSensei
**GitSensei** is a high-performance, local-first CLI tool that provides **sub-300ms** inline autocomplete for Git commit messages. 

It uses a local LLM (Llama 3.2 1B) to analyze your staged changes, applies an **aggressive minification bundler** to stay within context limits, and integrates directly into your shell completion.

## 🚀 Quick Start (Homebrew)

The easiest way to get GitSensei on macOS or Linux:

```bash
# 1. Install via Homebrew (Official)
brew install guyco3/tap/gitsensei

# 2. Initialize (Downloads Llama 3.2 1B via Ollama)
gitsensei init

# 3. Add the hook to your ~/.zshrc (or ~/.bashrc)
eval "$(gitsensei hook zsh)"
```

## 🛠️ Running Locally (Development)

If you want to build from source or contribute to the project:

### Prerequisites

- **Rust:** Install Rust (1.75+)
- **Ollama:** Download Ollama and ensure it's running.
```sh
brew install --cask ollama
```

### Setup

Clone the repo:

```bash
git clone https://github.com/guyco3/git-sensei.git
cd git-sensei
```

Build the binary:

```bash
cargo build --release
```

Install locally:

```bash
cargo install --path .
```

Test the Debug Mode: See how the aggressive bundler crushes your code before sending it to the LLM:

```bash
git add .
gitsensei --debug suggest "feat: "
```

## ⚙️ Configuration

You can customize GitSensei by editing the config file at `~/.config/gitsensei/config.toml`:

```toml
model = "llama3.2:1b"
endpoint = "http://localhost:11434/api/generate"
timeout_ms = 500
aggressive_minification = true
```

## 🔒 Privacy

GitSensei is 100% local. Your code never leaves your machine. It travels from your Git index to your local Ollama instance and nowhere else.

note: this is the end
