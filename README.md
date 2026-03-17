# 📄 CVapp

A fast, privacy-first resume and CV builder. All data processing and PDF generation happens entirely in your local browser — **no server, no sign-up, no data collection**.

![License](https://img.shields.io/badge/license-GPL--3.0-blue)
![Built with](https://img.shields.io/badge/built_with-React_19_%2B_Vite_8-61dafb)

---

## ✨ Features

| Feature | Description |
|---|---|
| **4 Professional Templates** | ATS Minimalist, Modern Creative, Timeline, and Timeline Two-Column |
| **Live Split-Screen Preview** | See changes in real-time as you type |
| **One-Click PDF Export** | Native `window.print()` — no third-party libraries |
| **ATS Hidden Keywords** | Inject invisible keywords for applicant tracking systems |
| **Multi-Language** | English, French, Arabic (with RTL support) |
| **Dark Mode** | Full dark theme across the entire UI |
| **Text Size Control** | Adjustable 80%–120% font scale slider |
| **LinkedIn Import** | Paste your LinkedIn JSON export to auto-fill fields |
| **Installable PWA** | Add to your home screen on any device |
| **100% Client-Side** | Zero backend — all data stays in your browser via `localStorage` |

---

## 🚀 Getting Started

### Prerequisites

- [Bun](https://bun.sh/) (recommended) or Node.js 18+

### Install & Run

```bash
# Clone the repo
git clone https://github.com/hevzer/cvapp.git
cd cvapp

# Install dependencies
bun install

# Start development server
bun run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build for Production

```bash
bun run build
bun run start
```

---

## 🏗️ Tech Stack

| Layer | Technology |
|---|---|
| Framework | Next.js 16 (App Router) + Vinext |
| Language | TypeScript |
| Styling | Tailwind CSS v4 |
| State | Zustand (persisted to localStorage) |
| Icons | Bootstrap Icons |
| PDF | Native `window.print()` with `@media print` CSS |
| PWA | Web Manifest + Service Worker |

---

## 📂 Project Structure

```
src/
├── app/                  # Next.js App Router pages & global CSS
├── components/
│   ├── forms/            # Input forms (PersonalInfo, Experience, etc.)
│   ├── templates/        # CV templates (ATS, Modern, Timeline, etc.)
│   └── ui/               # Reusable UI (Accordion, DarkModeToggle, etc.)
├── data/                 # Example data & i18n translations
├── lib/                  # i18n configuration
├── store/                # Zustand store (useResumeStore)
└── types/                # TypeScript type definitions
```

---

## 📄 Templates

| Template | Style |
|---|---|
| **ATS Minimalist** | Clean single-column, optimized for applicant tracking systems |
| **Modern Creative** | Two-column layout with visual flair and color accents |
| **Timeline** | Vertical timeline flow with a right sidebar |
| **Timeline Two-Column** | Dark sidebar with timeline in the main content area |

---

## 🔒 Privacy

CVapp is **fully client-side**. Your resume data never leaves your browser:

- No analytics or tracking
- No server-side processing
- All data stored in `localStorage`
- PDF generated via native browser print

---

## 📝 License

This project is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).
