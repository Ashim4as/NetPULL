# 🟢 NetPULL

**NetPULL** is a high-performance, full-stack video downloader built entirely in Rust. Designed with a sleek, hacker-inspired terminal aesthetic, it leverages `yt-dlp` under the hood to reliably extract and download media from YouTube, Instagram, and hundreds of other supported platforms.

![NetPULL Status](https://img.shields.io/badge/Status-ONLINE-brightgreen?style=for-the-badge&logo=terminal)
![Rust](https://img.shields.io/badge/Rust-100%25-orange?style=for-the-badge&logo=rust)

## ✨ Features

- ⚡ **Blazing Fast Backend:** Powered by `Axum` and `Tokio` for lightweight, asynchronous request handling.
- 🎨 **Terminal Aesthetic:** A beautiful, responsive WebAssembly (Wasm) frontend built with `Leptos` and `TailwindCSS` that mimics a professional hacker terminal.
- 📊 **Real-time Progress:** Uses Server-Sent Events (SSE) to stream live download progress (Percent, Speed, ETA) directly from the backend to the UI.
- ⚙️ **Quality Selection:** Choose your exact desired resolution (Best, 1080p, 720p, 480p) or extract Audio-Only (MP3/M4A) before downloading.
- 🗄️ **Persistent History:** Automatically logs all downloads (ID, Title, Status, Timestamp) into a local `SQLite` database, viewable in the UI.

## 🛠️ Tech Stack

### Backend
- **Rust**
- **Axum** (Web Framework)
- **Tokio** (Async Runtime)
- **SQLx** (SQLite Database)
- **yt-dlp** (Core extraction engine)

### Frontend
- **Rust** / **WebAssembly** (Wasm)
- **Leptos** (Reactive UI Framework)
- **Trunk** (Wasm build tool)
- **TailwindCSS** (Styling)

## 🚀 Getting Started

### Prerequisites
Before running NetPULL, ensure you have the following installed on your system:
- [Rust & Cargo](https://rustup.rs/)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) (Must be available in your system's PATH)
- [FFmpeg](https://ffmpeg.org/) (Required by yt-dlp for merging video and audio streams)

### Running the Project

You will need to run the backend and the frontend in two separate terminal instances.

#### 1. Start the Backend
Navigate to the backend directory, initialize the database, and run the server:
```bash
cd backend
# The SQLite database (yt-dl.db) will automatically initialize on first run
cargo run
```
*The backend will start on `http://127.0.0.1:3000`.*

#### 2. Start the Frontend
In a new terminal window, navigate to the frontend directory and start the Trunk development server:
```bash
cd frontend
trunk serve
```
*The frontend will compile to WebAssembly and open on `http://localhost:8080`.*

## 📂 Directory Structure

```text
NetPULL/
├── backend/                  # Axum API server
│   ├── src/
│   │   ├── handlers/         # Route handlers (video download, history)
│   │   ├── services/         # Core logic (yt-dlp process orchestration)
│   │   ├── models.rs         # Shared data structures
│   │   └── main.rs           # Server entry point & DB initialization
│   └── yt-dl.db              # SQLite Database (auto-generated)
├── frontend/                 # Leptos Wasm UI
│   ├── src/
│   │   ├── components/       # UI Components (Downloader, History, Header)
│   │   ├── api.rs            # SSE and REST API communication
│   │   ├── app.rs            # Main Application Layout
│   │   └── main.rs           # WebAssembly entry point
│   ├── index.html            # HTML template
│   └── Tailwind.css          # Tailwind configurations
└── downloads/                # Output directory for all downloaded media
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## 📜 License

This project is open-source and available under the [MIT License](LICENSE).
