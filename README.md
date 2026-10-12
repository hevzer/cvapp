# 📄 CVapp

A blazing-fast, privacy-first resume and CV builder written in **Rust**. Compiles natively to **Web (WebAssembly)**, **Windows Desktop**, and **Linux Desktop** — **no backend, no sign-up, no data collection**.

![License](https://img.shields.io/badge/license-GPL--3.0-blue)
![Built with](https://img.shields.io/badge/built_with-Rust_1.99_%2B_Leptos_0.8-DEA584)
![Desktop](https://img.shields.io/badge/desktop-Tauri_2-24C8DB)
![Styling](https://img.shields.io/badge/styling-Tailwind_CSS_v4-38bdf8)

---

## ✨ Features

| Feature | Description |
|---|---|
| **4 Professional Templates** | ATS Minimalist, Modern Creative, Timeline Creative, and Timeline Two-Column |
| **Live Split-Screen Preview** | Instant zero-latency reactive preview with Leptos signals |
| **Native A4 PDF Export** | Clean single-page output via browser native `window.print()` and CSS print engine |
| **Cross-Platform** | Web (WASM PWA), Windows (`.exe` / `.msi`), and Linux (`.deb` / AppImage) |
| **ATS Hidden Keywords** | Inject invisible keywords to maximize ATS score matching |
| **Multi-Language (i18n)** | English and French UI with fallback detection |
| **LinkedIn Archive Importer** | Native pure-Rust ZIP and JSON parser extracting profile, positions, and skills |
| **Interactive Style Controls** | Typography font scale, spacing scale, text zoom, accent colors, and dark mode |
| **100% Client-Side Privacy** | Zero telemetry, zero external APIs — data persisted safely in local storage |

---

## 🚀 Getting Started

### Prerequisites

We recommend using [mise](https://mise.jdx.dev/) to automatically manage tools and environment:

```bash
# Clone the repository
git clone https://github.com/hevzer/cvapp.git
cd cvapp

# Install toolchains via mise (Rust 1.99, Trunk, Tauri CLI, Bun)
mise install
```

Alternatively, ensure you have:
- [Rust](https://rustup.rs/) (1.80+ or latest stable) with target `wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [Bun](https://bun.sh/) (for Tailwind CSS v4 CLI)
- [Tauri CLI](https://v2.tauri.app/) (`cargo install tauri-cli`)

---

## 🛠️ Development & Building

### Web (WebAssembly)

```bash
# Start Leptos web development server with hot-reloading
trunk serve
# or: mise run dev

# Build optimized WebAssembly production distribution in dist/
trunk build --release
# or: mise run build
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

### Desktop (Windows & Linux with Tauri 2)

```bash
# Run desktop app in development mode
cargo tauri dev
# or: mise run desktop:dev

# Build production desktop installers
cargo tauri build
# or: mise run desktop:build
```

### Running Tests

```bash
cargo test --lib
# or: mise run test
```

---

## 🏗️ Architecture & Tech Stack

| Layer | Technology |
|---|---|
| **Core Language** | Rust 2024 edition |
| **Web Framework** | [Leptos 0.8](https://leptos.dev/) (Client-Side Rendering / WASM) |
| **Desktop Shell** | [Tauri 2](https://v2.tauri.app/) (WebView2 on Windows, WebKitGTK on Linux) |
| **WASM Bundler** | [Trunk 0.21](https://trunkrs.dev/) with automated Tailwind pre-build hooks |
| **Styling** | [Tailwind CSS v4](https://tailwindcss.com/) with `@media print` utilities |
| **Tooling Manager** | [mise-en-place](https://mise.jdx.dev/) (`mise.toml`) |
| **Archive Parsing** | Pure-Rust `zip` with `zlib-rs` decompression & `serde_json` |
| **Storage** | Client-side reactive `LocalStorage` synced with Zustand-compatible JSON format |

---

## 📂 Project Structure

```
.
├── Cargo.toml               # Root Cargo workspace & dependencies
├── Trunk.toml               # Web bundler config & Tailwind hooks
├── mise.toml                # Pinned tool versions and tasks
├── tailwind.css             # Tailwind v4 theme, animations & print rules
├── index.html               # Entry HTML for Trunk WASM mounting
├── src/
│   ├── lib.rs               # Library entry point & WASM startup
│   ├── app.rs               # Main App shell (header, sidebar, preview)
│   ├── models.rs            # Resume serde data models & types
│   ├── state.rs             # Reactive AppState & LocalStorage sync
│   ├── i18n.rs              # Dictionaries (EN, FR) & language options
│   ├── date.rs              # Numeric & localized date formatting
│   ├── services/
│   │   └── linkedin.rs      # Pure-Rust ZIP and JSON archive parser
│   ├── data/
│   │   └── example.rs       # Alexandra Chen default demo resume
│   ├── templates/
│   │   ├── ats_minimalist.rs
│   │   ├── modern_creative.rs
│   │   ├── timeline_creative.rs
│   │   ├── timeline_two_column.rs
│   │   └── contact_block.rs
│   └── components/
│       ├── about_modal.rs
│       ├── data_actions.rs
│       ├── export_button.rs
│       ├── linkedin_modal.rs
│       ├── forms/           # Form sections for all CV domains
│       └── ui/              # Sliders, accordion, input wrappers
└── src-tauri/               # Tauri 2 native desktop configuration & harness
```

---

## 🔒 Privacy & Offline First

CVapp is **strictly client-side**:
- Zero analytics, telemetry, or third-party tracking
- Zero server-side API processing or storage
- All data stored safely on your machine
- PDF generated via native browser print engine

---

## 📝 License

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).
