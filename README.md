# 🥧Crunchypi

**Crunchypi** is a desktop application that combines the power of local LLMs with a glassmorphic user interface. Built to solve math problems

---

## Features

- **Local AI Intelligence**: Powered by `qwen2-math:1.5b` via Ollama for private, offline math solving.
- **Beautiful Math Rendering**: Renders complex LaTeX equations instantly using KaTeX.
- **Glassmorphic Design**: A "Midnight" glassmorphism theme with smooth gradients, blur effects, and custom scrollbars.
- **Smart Interaction**: Copy raw LaTeX answers with a single click.

---

## Tech Stack

- **Core**: [Tauri](https://tauri.app/) (Rust 🦀)
- **Frontend**: [Svelte](https://svelte.dev/) + [Vite](https://vitejs.dev/)
- **Styling**: Vanilla CSS (CSS Grid + Flexbox)
- **Math Engine**: [KaTeX](https://katex.org/)
- **AI Backend**: [Ollama](https://ollama.ai/)

---

## Getting Started

### Prerequisites

1. **Ollama**: Install [Ollama](https://ollama.ai/) and pull the math model:

    ```bash
    ollama pull qwen2-math:1.5b
    ```

2. **System Dependencies** (Linux only):
    - **Arch Linux**: `sudo pacman -S webkit2gtk-4.1`
    - **Debian/Ubuntu**: `sudo apt install libwebkit2gtk-4.1-dev`

### Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/KleaSCM/crunchypi.git
cd crunchypi
npm install
```

### Running

Start the development server:

```bash
npm run tauri dev
```
