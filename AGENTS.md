# CVapp - AI Agent Development Log

This file, `AGENTS.md`, serves as a permanent architectural and historical record of the AI assistant's work on **CVapp**, designed to provide context for future LLM assistants working on this application.

## Application Overview
**CVapp** is a privacy-first, fully client-side resume and CV builder. It features a split-screen live preview, multiple professional template architectures, automated ATS hidden keyword injection, and robust multi-language translation support. 

Because it operates entirely without a backend API, all state is handled natively through Zustand localStorage persistence, and all PDF generation relies purely on the browser's native `window.print()` engine scaling through CSS `@media print` utilities.

## Technology Stack
- **Framework**: Vite 8 + React 19 (single-page application, no SSR)
- **Entry points**: `index.html` (root) → `src/main.tsx` (mounts `<ResumeBuilder />` into `#root`)
- **Language**: TypeScript
- **Styling**: Tailwind CSS v4 via `@tailwindcss/postcss` (Vite auto-loads `postcss.config.mjs`) with dark mode and print variant support
- **State Management**: Zustand (with Persist middleware)
- **Icons**: Bootstrap Icons — CSS imported via ES module (`import 'bootstrap-icons/font/bootstrap-icons.css'`) in templates that use it
- **Fonts**: Google Fonts loaded via `<link>` in `index.html` (Inter, Roboto, Outfit, Lora, Merriweather). `--font-sans: "Inter", system-ui, sans-serif` in `src/globals.css`.
- **Installability**: Progressive Web App. `public/manifest.json` is referenced from `index.html`; `public/sw.js` is registered from `src/main.tsx` on window `load`.

## Key Technical Decisions & Quirks

1. **PDF Generation (No Libraries)**: 
   - *History*: The application originally attempted to use `html2pdf.js`, but this failed silently when attempting to parse modern Tailwind CSS v4 functionalities (such as `oklab` color functions native to CSS).
   - *Resolution*: The app was refactored to use native `window.print()`. We utilize `print:hidden` to strip UI elements and enforce tight component scaling parameters to fit perfectly onto A4 paper dimensions using precise Tailwind print typography limits. **Future agents should avoid introducing canvas-based PDF generation libraries.**

2. **Template Architectures**:
   - `ATSMinimalist`: Strict vertical semantic HTML flow. ATS-compliant and explicitly unstyled for maximal parseability.
   - `ModernCreative`: Two-column flex layout designed for visual flair with color accents.
   - `TimelineCreative`: Two-column layout with vertical timeline dots in the left column and a right sidebar for skills/languages.
   - `TimelineTwoColumn`: Dark sidebar (left) with contact/skills, main content (right) with timeline.
   - **A4 Print Fitting**: Templates use `max-h-[297mm] overflow-hidden` on the root container to guarantee single-page PDF output. All spacing and typography are set to compact values that natively fit A4 dimensions — **do not use `print:` overrides for sizing**, as this creates Chromium flexbox bugs. Design for A4 on screen; the print output will match.

3. **Internationalization (i18n)**:
   - Translation data is stored natively inside `src/lib/i18n.ts`. 
   - Language switching handles standard boilerplate headings. 
   - *Crash Warning*: We actively removed defunct languages (`it`, `de`, `es`). Future agents modifying the `cvLanguage` state variable in Zustand must implement a valid fallback check (found in `ResumeBuilder.tsx`) to prevent fatal application crashes when migrating user caches between versions.

4. **Branding Assets**:
   - Integrated a customized, natively cropped MacOS style "squircle" logo for use throughout the standard viewport and PWA configurations.

5. **Text Scale Control**:
   - A `textScale` number (0.8–1.2) is stored in the Zustand store and persisted to localStorage.
   - The `TextScaleSlider` component renders a range input in the sidebar.
   - Scaling is applied via CSS `zoom` on the `#cv-preview` container (not `font-size: em`, which doesn't cascade to children with hardcoded `px` values).

6. **GitHub Integration**:
   - Personal info forms support a GitHub profile URL field.
   - Templates that display contact info render a GitHub icon/link when provided.

## Zustand Store Shape (`useResumeStore`)
Key state fields:
- `resumeData` — all CV content (personalInfo, experience, education, skills, languages, hiddenKeywords, cvLanguage)
- `activeTemplate` — `'ats' | 'modern' | 'timeline' | 'timelineTwoColumn'`
- `darkMode` — boolean
- `textScale` — number (default `1`, range `0.8`–`1.2`)

## Maintenance Commands
```bash
bun run dev      # Start Vite dev server (default: http://localhost:5173)
bun run build    # Type-check (tsc --noEmit) then build to dist/
bun run preview  # Serve the production build locally
bun run lint     # ESLint (flat config: typescript-eslint + react-hooks)
bun update       # Increment safe dependency versions
```

Deploy the contents of `dist/` to any static host (Cloudflare Pages, Netlify, GitHub Pages, S3+CF, etc.). No server runtime required.
