# 🎬 Redner OS

**A Headless, Collaborative Creative Engine & Platform built in Rust.**

Redner is not just another video editor—it is a **Creative Operating System**. It is a headless, API-first timeline engine that treats video editing as code, implements Figma-style real-time collaboration, and provides a secure WebAssembly (Wasm) sandbox for third-party plugins and Autonomous AI agents.

---

## 🚀 The Vision

Modern creative tools are closed boxes. They trap assets in proprietary files, force linear rendering, and isolate creators. Redner is built differently, guided by three core philosophies:

1. **Figma for Video:** Real-time presence, timeline comments, and operation-based project syncing over WebSockets.
2. **VS Code for Creativity:** An open `Platform SDK` allowing anyone to write custom Wasm plugins (in JS, Rust, or Go) to extend the engine safely.
3. **Git for Timelines:** AI edits and human edits are treated as versioned commits. Don't like an AI's pacing adjustment? Revert to the previous timeline branch seamlessly.

---

## 🏗️ System Architecture

Redner's backend is heavily decoupled and layered like a traditional operating system:

| Layer | Domain | Description |
| --- | --- | --- |
| **5. Network** | `src/cloud` | Operation-based sync engine, Axum WebSockets, JWT Identity, Org RBAC. |
| **4. API / SDK** | `src/platform` | Extism Wasm runtime, Tool Registry, JSON Schema validation. |
| **3. Hardware Abstraction** | `src/performance` & `src/runtime` | Node-graph execution, async task pipelines (ElevenLabs, Rhubarb LipSync). |
| **2. Logic Unit** | `src/ai` | Agentic AI Memory, explicit creator preferences, LLM prompting logic. |
| **1. The Kernel** | `src/db` & `src/repos` | Headless timeline abstractions, SQLx Postgres data structures. |

---

## ✨ Core Features

* **Headless Timeline Engine:** Edit media programmatically. Tracks, clips, and assets are strictly typed data structures, not proprietary files.
* **Real-Time Multiplayer:** Uses `tokio::sync::broadcast` and Axum WebSockets to transmit tiny `SyncOperation` packets, enabling real-time cursors and collaborative editing with zero file-locking.
* **WebAssembly Plugin Sandbox:** Execute untrusted 3rd-party code safely. Plugins compile to Wasm and interact with the Redner SDK via secure memory passing.
* **Agentic AI Integration:** AI doesn't just guess; it reads a Creator's explicit `AiMemory` (preferred pacing, caption styles) and executes edits through the same SDK human users do.
* **Version Control (Git for Video):** Every edit is an atomic operation. Checkpoint your timeline, branch experiments, and roll back AI-generated edits instantly.
* **Organization Workspaces:** Full Role-Based Access Control (RBAC) allowing teams to share brand kits, Digital Twins, and projects securely.

---

## 🛠️ Technology Stack

* **Language:** [Rust](https://www.rust-lang.org/) (Strictly typed, memory safe, blazingly fast)
* **Web Server:** [Axum](https://github.com/tokio-rs/axum) & [Tokio](https://tokio.rs/) (Async runtime)
* **Database:** PostgreSQL with [SQLx](https://www.google.com/search?q=https://github.com/launchbadge/sqlx) (Compile-time query verification)
* **Plugin Runtime:** [Extism](https://extism.org/) (Universal WebAssembly Sandbox)
* **Authentication:** JWT validation via frontend managed-auth (e.g., Supabase/Clerk)
* **Real-Time:** Axum WebSockets

---

## 💻 Getting Started (Local Development)

### 1. Prerequisites

Ensure you have the following installed:

* [Rust & Cargo](https://rustup.rs/)
* [PostgreSQL](https://www.postgresql.org/download/)
* [Extism CLI](https://extism.org/docs/install/) (for compiling Wasm plugins)

### 2. Environment Setup

Clone the repository and set up your environment variables:

```bash
git clone https://github.com/yourusername/redner-backend.git
cd redner-backend
cp .env.example .env

```

Populate your `.env` file:

```env
# Database
DATABASE_URL=postgres://user:password@localhost:5432/redner_db

# Server
PORT=8080
RUST_LOG=info,redner=debug

# Security & Cloud APIs
JWT_SECRET=your_super_secret_jwt_key
ELEVENLABS_API_KEY=your_api_key
GEMINI_API_KEY=your_api_key

```

### 3. Database Migrations

Run the SQLx migrations to build the Workspace and Cloud tables:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
sqlx database setup

```

### 4. Run the Engine

Fire up the backend server:

```bash
cargo run

```

You should see:

```text
🚀 Redner Backend listening on http://127.0.0.0:8080

```

---

## 🔌 API Overview

Redner exposes a REST API merged with high-performance WebSockets.

### Identity & Collaboration

* `GET /api/v1/identity/profile` - Fetch the authenticated Creator's global settings.
* `GET /api/v1/collab/:project_id/connect` - Upgrade to a WebSocket for real-time `SyncOperation` broadcasting.

### Projects & Rendering

* `POST /api/v1/projects` - Initialize a headless timeline.
* `POST /api/v1/versions/:project_id/commits` - Create a checkpoint in the project's operation log.
* `POST /api/v1/runtime/execute` - Trigger the parallel Node-Graph execution engine for rendering/processing.

---

## 🤝 Contributing

Redner is an ambitious step forward for creative tooling. Whether you are optimizing the Node-Graph runtime, adding new Wasm targets, or refining the AI Agent memory schemas, your PRs are welcome.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.