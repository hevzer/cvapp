# CVapp - AI Agent Development Log

This file, `AGENTS.md`, serves as a permanent architectural and historical record of the AI assistant's work on **CVapp**, designed to provide context for future LLM assistants working on this application.

## Application Overview
**CVapp** is a privacy-first, fully client-side resume and CV builder written in **Rust**. It features a zero-latency split-screen live preview, multiple professional template architectures, automated ATS hidden keyword injection, robust multi-language translation support, and native LinkedIn archive import.

Because it operates entirely without a backend API, all state is handled natively through reactive `RwSignal`s with `gloo-storage` LocalStorage persistence (compatible with legacy Zustand JSON schema), and all PDF generation relies purely on the browser's native `window.print()` engine scaling through CSS `@media print` utilities.

## Technology Stack (Rust Rewrite)
- **Language**: Rust (2024 edition, 1.99.0 pinned via `mise.toml`)
- **Web Framework**: [Leptos 0.8.22](https://leptos.dev/) (Client-Side Rendering / WebAssembly)
- **Desktop Framework**: [Tauri 2.12.3](https://v2.tauri.app/) (native desktop shell for Windows WebView2 & Linux WebKitGTK)
- **WASM Bundler**: [Trunk 0.21.14](https://trunkrs.dev/) with automated pre-build Tailwind compilation hook
- **Tooling & Environment**: [mise](https://mise.jdx.dev/) (`mise.toml`)
- **Styling**: Tailwind CSS v4 via `@tailwindcss/cli` (`tailwind.css` -> `style/tailwind.css`) with dark mode, animations, and print variant support
- **Icons**: Bootstrap Icons loaded via CDN in `index.html`
- **Fonts**: Google Fonts loaded via `<link>` in `index.html` (Inter, Roboto, Outfit, Lora, Merriweather)
- **PWA**: `manifest.json` and service worker registered on startup
- **Archive Parsing**: Pure-Rust `zip` 9.0.2 with `zlib-rs` decompression & `serde_json`

## Key Technical Decisions & Quirks

1. **PDF Generation (Native `window.print()`)**:
   - The app uses native `window.print()`, triggered via `ExportButton`.
   - Never introduce canvas-based or heavy PDF libraries (e.g. `html2pdf.js` or `jspdf`).
   - We utilize `print:hidden` to strip UI controls and enforce single-page A4 constraints.

2. **Template Architectures**:
   - `ATSMinimalist`: Strict vertical semantic HTML flow. ATS-compliant and explicitly unstyled for maximal parseability.
   - `ModernCreative`: Two-column grid layout with color accents.
   - `TimelineCreative`: Two-column layout with vertical timeline dots in the left column and a right sidebar for skills/languages.
   - `TimelineTwoColumn`: Dark sidebar (left) with contact/skills, main content (right) with timeline.
   - **A4 Print Fitting**: Templates use `max-h-[297mm] overflow-hidden` on the root container to guarantee single-page PDF output. Spacing and typography are set to compact values that natively fit A4 dimensions.

3. **Internationalization (i18n)**:
   - Translation data is stored natively inside `src/i18n.rs`.
   - Supported languages: English (`en`) and French (`fr`). Defunct languages (`it`, `de`, `es`) fall back gracefully to English.

4. **Branding Assets**:
   - Custom macOS style squircle logo (`public/cvapp_macos_logo.png`).

5. **Text Scale & Spacing Controls**:
   - `text_scale` (0.8–1.2), `spacing_scale` (0.7–1.3), `font_scale` (0.8–1.2) are managed in `AppState`.
   - Text scaling is applied via CSS `zoom` on the `#cv-preview` container.
   - Spacing and typography font scales are applied via CSS variables `--ss` and `--fs` defined on `#cv-preview`.

6. **State Storage Compatibility**:
   - Stored in `localStorage` under key `"cvapp-storage"`.
   - Uses `ZustandWrapper` `{ state: PersistedState, version: Option<u32> }` to maintain 100% backward and forward compatibility with prior user caches.

7. **LinkedIn Importer**:
   - `src/services/linkedin.rs` provides a pure-Rust parser for both `.zip` archive exports and `.json` profile dumps.
   - Employs streaming file reader with JavaScript `Uint8Array` bridging.

8. **Live JSON File Sync & Debounced Persistence**:
   - Continuous auto-save is powered by a reactive debounced Leptos `Effect` (~300ms) in `src/app.rs` tracking all state signals without component boilerplate.
   - Saves synchronously to `localStorage` (via `ZustandWrapper`) as an always-active fail-safe backup.
   - Supports **Project File Workflow** via `src/services/file_sync.rs`:
     - **Desktop (Tauri)**: Invokes native Rust commands with `rfd` file pickers (`pick_save_file`, `pick_open_file`) and writes directly to disk via `write_file_content`.
     - **Web (Chromium)**: Invokes browser File System Access API (`showSaveFilePicker`, `showOpenFilePicker`) with streaming writable file handles.
     - **Fallback**: Gracefully falls back to browser cache auto-save and standard manual JSON file download/upload.

9. **Projects System & Home Dashboard**:
   - Multi-resume management powered by `CvProject` and `ProjectsStore` in `src/models.rs` and `src/state.rs`.
   - Projects Dashboard view (`src/components/home/projects_home.rs`) allows creating new blank resumes, spinning up from example data, importing JSON / LinkedIn, duplicating, renaming, exporting, and deleting resumes.
   - Header project switcher allows instant switching between resumes directly within the editor.
   - Transparent zero-data-loss migration from legacy single-resume `cvapp-storage` to `cvapp-projects`.

10. **CLI & Model Context Protocol (MCP) Server (`crates/cvapp-cli`)**:
   - Built specifically for autonomous AI agents and terminal workflows.
   - **CLI Mode**: Inspect, manipulate, ATS-optimize, and export resumes:
     - `mise run cli -- project list`
     - `mise run cli -- resume set-profile --name "Jane Doe" --title "Staff Engineer"`
     - `mise run cli -- ats analyze --job-text "..."` (calculates match % and identifies missing keywords)
     - `mise run cli -- ats inject Rust Kubernetes WebAssembly`
     - `mise run cli -- export html -o resume.html`
   - **MCP Server Mode (`cvapp mcp`)**: Standard JSON-RPC 2.0 stdio transport (spec 2024-11-05).
     - Configured automatically in `.agents/mcp_config.json`.
     - Exposes tools to AI agents: `cvapp_list_projects`, `cvapp_get_project`, `cvapp_create_project`, `cvapp_duplicate_project`, `cvapp_update_personal_info`, `cvapp_update_summary`, `cvapp_add_experience`, `cvapp_set_skills`, `cvapp_inject_ats_keywords`, `cvapp_analyze_job_fit`, `cvapp_set_template`, `cvapp_export_resume`, `cvapp_import_linkedin`.
     - Exposes resources: `cvapp://templates`, `cvapp://projects`, `cvapp://projects/{id}`.

## Maintenance Commands
```bash
# Using mise (recommended):
mise run setup          # Auto-detect OS and install required desktop system libraries
mise run dev            # Start Trunk dev server with hot-reloading (http://localhost:8080)
mise run build          # Build release WASM bundle to dist/
mise run desktop:dev    # Start Tauri desktop app in dev mode
mise run desktop:build  # Build release desktop installer
mise run test           # Run Rust web unit tests
mise run cli -- --help  # Run CVapp CLI
mise run mcp            # Start Model Context Protocol server on stdio
mise run cli:test       # Run CLI and MCP integration test suite
mise run cli:build      # Compile release CLI / MCP binary

# Or using cargo / trunk directly:
trunk serve
trunk build --release
cargo test --lib
cargo test --manifest-path crates/cvapp-cli/Cargo.toml
cargo tauri build
```

Deploy the contents of `dist/` to any static host (Cloudflare Pages, Netlify, GitHub Pages, S3+CF, etc.). No server runtime required.
