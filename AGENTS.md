# CVapp - AI Agent Development Log

This file, `AGENTS.md`, serves as a permanent architectural and historical record of the AI assistant's work on **CVapp**, designed to provide context for future LLM assistants working on this application.

## Application Overview
**CVapp** is a privacy-first, fully client-side resume and CV builder. It features a split-screen live preview, multiple professional template architectures, automated ATS hidden keyword injection, and robust multi-language translation support. 

Because it operates entirely without a backend API, all state is handled natively through Zustand localStorage persistence, and all PDF generation relies purely on the browser's native `window.print()` engine scaling through CSS `@media print` utilities.

## Technology Stack
- **Framework**: Next.js (App Router pattern) + `vinext` for high-performance builds
- **Language**: TypeScript
- **Styling**: Tailwind CSS v4 (with dark mode and print variant support)
- **State Management**: Zustand (with Persist middleware)
- **Icons**: Bootstrap Icons (via CDN/SVG)
- **Installability**: Progressive Web App (PWA) configured via Web Manifest and Service Worker interceptors.

## Key Technical Decisions & Quirks

1. **PDF Generation (No Libraries)**: 
   - *History*: The application originally attempted to use `html2pdf.js`, but this failed silently when attempting to parse modern Tailwind CSS v4 functionalities (such as `oklab` color functions native to CSS).
   - *Resolution*: The app was refactored to use native `window.print()`. We utilize `print:hidden` to strip UI elements and enforce tight component scaling parameters to fit perfectly onto A4 paper dimensions using precise Tailwind print typography limits. **Future agents should avoid introducing canvas-based PDF generation libraries.**

2. **Template Architectures**:
   - `ATSMinimalist`: Strict vertical semantic HTML flow. ATS-compliant and explicitly unstyled for maximal parseability.
   - `ModernCreative` & `TimelineCreative`: Two-column flex layouts designed for visual flair.
   - *Note*: Ensure that extensive padding/margins are not introduced in generic elements, as this immediately causes A4 print overflows.

3. **Internationalization (i18n)**:
   - Translation data is stored natively inside `src/data/i18n.ts`. 
   - Language switching handles standard boilerplate headings. 
   - *Crash Warning*: We actively removed defunct languages (`it`, `de`, `es`). Future agents modifying the `cvLanguage` state variable in Zustand must implement a valid fallback check (found in `ResumeBuilder.tsx`) to prevent fatal application crashes when migrating user caches between versions.

4. **Branding Assets**:
   - Integrated a customized, natively cropped MacOS style "squircle" logo for use throughout the standard viewport and PWA configurations.

## Maintenance Commands
```bash
bun run dev  # Start vinext development server
bun update   # Increment safe dependency versions
```
