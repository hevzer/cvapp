'use client';

import { useRef, useEffect, useState } from 'react';
import { useResumeStore } from '../store/useResumeStore';
import Accordion from './ui/Accordion';
import TemplateToggle from './ui/TemplateToggle';
import TextScaleSlider from './ui/TextScaleSlider';
import SpacingSlider from './ui/SpacingSlider';
import FontScaleSlider from './ui/FontScaleSlider';
import StyleCustomizer from './ui/StyleCustomizer';
import DarkModeToggle from './ui/DarkModeToggle';
import PersonalInfoForm from './forms/PersonalInfoForm';
import SummaryForm from './forms/SummaryForm';
import ExperienceForm from './forms/ExperienceForm';
import AboutModal from './AboutModal';
import EducationForm from './forms/EducationForm';
import CertificationsForm from './forms/CertificationsForm';
import TechnicalSkillsForm from './forms/TechnicalSkillsForm';
import SoftSkillsForm from './forms/SoftSkillsForm';
import LanguagesForm from './forms/LanguagesForm';
import HiddenKeywordsForm from './forms/HiddenKeywordsForm';
import { i18n, getSafeLanguage, languageOptions } from '../lib/i18n';
import ATSMinimalist from './templates/ATSMinimalist';
import ModernCreative from './templates/ModernCreative';
import TimelineCreative from './templates/TimelineCreative';
import TimelineTwoColumn from './templates/TimelineTwoColumn';
import ExportButton from './ExportButton';
import LinkedInImport from './LinkedInImport';
import DataActions from './ui/DataActions';

export default function ResumeBuilder() {
  const activeTemplate = useResumeStore((s) => s.activeTemplate);
  const loadExampleData = useResumeStore((s) => s.loadExampleData);
  const clearAllData = useResumeStore((s) => s.clearAllData);
  const darkMode = useResumeStore((s) => s.darkMode);
  const cvLang = getSafeLanguage(useResumeStore((s) => s.resumeData.cvLanguage));
  const setCvLang = useResumeStore((s) => s.setCvLanguage);
  const previewRef = useRef<HTMLDivElement>(null);
  const textScale = useResumeStore((s) => s.textScale);
  const spacingScale = useResumeStore((s) => s.spacingScale);
  const fontScale = useResumeStore((s) => s.fontScale);
  const accentColor = useResumeStore((s) => s.accentColor);
  const sidebarColor = useResumeStore((s) => s.sidebarColor);
  const fontFamily = useResumeStore((s) => s.fontFamily);
  const [hydrated, setHydrated] = useState(false);

  useEffect(() => {
    setHydrated(true);
  }, []);

  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [darkMode]);

  if (!hydrated) {
    return (
      <div className="flex items-center justify-center h-screen bg-gradient-to-br from-slate-50 to-gray-100 dark:from-gray-900 dark:to-gray-950">
        <div className="flex flex-col items-center gap-4 animate-fade-in">
          <div className="w-14 h-14 rounded-2xl bg-gradient-to-br from-cyan-500 via-purple-500 to-pink-500 flex items-center justify-center shadow-xl shadow-cyan-500/20 animate-pulse">
            <span className="text-white font-extrabold text-xl tracking-tight">CV</span>
          </div>
          <div className="flex items-center gap-3 text-gray-400">
            <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.37 0 0 5.37 0 12h4z" />
            </svg>
            <span className="text-sm font-medium tracking-wide">Loading CVapp…</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen bg-slate-100 dark:bg-black print:bg-none print:h-auto print:block">
      {/* ── Header ── */}
      <header className="flex-shrink-0 bg-white dark:bg-[#0a0a0a] shadow-md z-20 px-6 py-3 animate-slide-up print:hidden relative border-b border-transparent dark:border-white/[0.05]">
        <div className="flex items-center justify-between max-w-[1920px] mx-auto">
          {/* Brand */}
          <div className="flex items-center gap-3 group cursor-default select-none">
            <div className="w-11 h-11 flex items-center justify-center transition-shadow duration-300">
              <img src="/cvapp_macos_logo.png" alt="CVapp Logo" className="w-full h-full object-contain drop-shadow-md" />
            </div>
            <h1 className="text-lg font-extrabold tracking-tight text-gray-900 dark:text-white">
              CVapp
            </h1>
          </div>

          {/* Actions */}
          <div className="flex items-center gap-1.5">
            <AboutModal />
            <button
              type="button"
              onClick={loadExampleData}
              className="px-4 py-2 text-[11px] font-bold uppercase tracking-widest rounded-full text-cyan-700 bg-cyan-100 dark:text-cyan-300 dark:bg-cyan-900/50 hover:bg-cyan-200 dark:hover:bg-cyan-800/60 hover:shadow-sm active:scale-95 transition-all duration-200"
            >
              ✨ Fill Example
            </button>
            <button
              type="button"
              onClick={clearAllData}
              className="px-4 py-2 text-[11px] font-bold uppercase tracking-widest rounded-full text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-700 dark:hover:text-slate-300 active:scale-95 transition-all duration-200"
            >
              Clear All
            </button>
            <div className="w-px h-5 bg-gray-200 dark:bg-white/10 mx-1.5" />
            <select
              value={cvLang}
              onChange={(e) => setCvLang(e.target.value)}
              className="text-[12px] font-bold uppercase tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 rounded-full px-3 py-2 outline-none border border-transparent focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all hover:bg-slate-200 dark:hover:bg-slate-700 cursor-pointer appearance-none shadow-sm"
              style={{ paddingRight: '2rem', backgroundImage: `url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e")`, backgroundPosition: 'right 0.25rem center', backgroundRepeat: 'no-repeat', backgroundSize: '1.5em 1.5em' }}
            >
              {languageOptions.map((opt) => (
                <option key={opt.code} value={opt.code}>
                  {opt.label}
                </option>
              ))}
            </select>
            <div className="w-px h-5 bg-gray-200 dark:bg-white/10 mx-1.5" />
            <DarkModeToggle />
          </div>
        </div>
      </header>

      {/* ── Main ── */}
      <div className="flex flex-1 overflow-hidden print:block print:overflow-visible">
        {/* Left Panel — Forms */}
        <aside className="w-[440px] flex-shrink-0 overflow-y-auto border-r border-slate-200 dark:border-white/[0.05] bg-slate-50 dark:bg-[#0a0a0a] p-5 animate-stagger space-y-4 print:hidden z-10 shadow-xl">
          <TemplateToggle />
          <TextScaleSlider />
          <SpacingSlider />
          <FontScaleSlider />
          <StyleCustomizer />
          <LinkedInImport />

          <Accordion title={i18n[cvLang].profile} icon="👤" defaultOpen={true}>
            <PersonalInfoForm />
          </Accordion>

          <Accordion title={i18n[cvLang].profile + ' (Summary)'} icon="📝" defaultOpen={true}>
            <SummaryForm />
          </Accordion>

          <Accordion title={i18n[cvLang].experience} icon="💼">
            <ExperienceForm />
          </Accordion>

          <Accordion title={i18n[cvLang].education} icon="🎓">
            <EducationForm />
          </Accordion>

          <Accordion title={i18n[cvLang].certifications} icon="📜">
            <CertificationsForm />
          </Accordion>

          <Accordion title={i18n[cvLang].technicalSkills} icon="💻">
            <TechnicalSkillsForm />
          </Accordion>

          <Accordion title={i18n[cvLang].softSkills} icon="🤝">
            <SoftSkillsForm />
          </Accordion>

          <Accordion title={i18n[cvLang].languages} icon="🌍">
            <LanguagesForm />
          </Accordion>

          <Accordion title="ATS Hidden Keywords" icon="🔍">
            <HiddenKeywordsForm />
          </Accordion>

          <div className="pt-2">
            <DataActions />
            <ExportButton targetRef={previewRef} />
          </div>
        </aside>

        {/* Right Panel — Preview */}
        <main className="flex-1 overflow-y-auto bg-slate-200 dark:bg-black p-8 print:p-0 print:m-0 print:bg-none print:overflow-visible flex flex-col items-center">
          <style>
            {`
              #cv-preview {
                --ss: ${spacingScale};
                --fs: ${fontScale};
              }
              
              /* Typography Scaling */
              #cv-preview .text-\\[9px\\] { font-size: calc(9px * var(--fs)) !important; }
              #cv-preview .text-\\[10px\\] { font-size: calc(10px * var(--fs)) !important; }
              #cv-preview .text-\\[11px\\] { font-size: calc(11px * var(--fs)) !important; }
              #cv-preview .text-\\[11\\.5px\\] { font-size: calc(11.5px * var(--fs)) !important; }
              #cv-preview .text-\\[12px\\] { font-size: calc(12px * var(--fs)) !important; }
              #cv-preview .text-\\[12\\.5px\\] { font-size: calc(12.5px * var(--fs)) !important; }
              #cv-preview .text-\\[13px\\] { font-size: calc(13px * var(--fs)) !important; }
              #cv-preview .text-xs { font-size: calc(0.75rem * var(--fs)) !important; }
              #cv-preview .text-sm { font-size: calc(0.875rem * var(--fs)) !important; }
              #cv-preview .text-base { font-size: calc(1rem * var(--fs)) !important; }
              #cv-preview .text-lg { font-size: calc(1.125rem * var(--fs)) !important; }
              #cv-preview .text-xl { font-size: calc(1.25rem * var(--fs)) !important; }
              #cv-preview .text-2xl { font-size: calc(1.5rem * var(--fs)) !important; }
              #cv-preview .text-3xl { font-size: calc(1.875rem * var(--fs)) !important; }

              /* Spacing Scaling */
              #cv-preview .mb-1 { margin-bottom: calc(0.25rem * var(--ss)) !important; }
              #cv-preview .mb-1\\.5 { margin-bottom: calc(0.375rem * var(--ss)) !important; }
              #cv-preview .mb-2 { margin-bottom: calc(0.5rem * var(--ss)) !important; }
              #cv-preview .mb-2\\.5 { margin-bottom: calc(0.625rem * var(--ss)) !important; }
              #cv-preview .mb-3 { margin-bottom: calc(0.75rem * var(--ss)) !important; }
              #cv-preview .mb-3\\.5 { margin-bottom: calc(0.875rem * var(--ss)) !important; }
              #cv-preview .mb-4 { margin-bottom: calc(1rem * var(--ss)) !important; }
              #cv-preview .mb-5 { margin-bottom: calc(1.25rem * var(--ss)) !important; }
              #cv-preview .mb-6 { margin-bottom: calc(1.5rem * var(--ss)) !important; }
              
              #cv-preview .pb-1 { padding-bottom: calc(0.25rem * var(--ss)) !important; }
              #cv-preview .pb-1\\.5 { padding-bottom: calc(0.375rem * var(--ss)) !important; }
              #cv-preview .pb-2 { padding-bottom: calc(0.5rem * var(--ss)) !important; }
              #cv-preview .pb-3 { padding-bottom: calc(0.75rem * var(--ss)) !important; }
              #cv-preview .pb-4 { padding-bottom: calc(1rem * var(--ss)) !important; }
              
              #cv-preview .mt-0 { margin-top: 0 !important; }
              #cv-preview .mt-0\\.5 { margin-top: calc(0.125rem * var(--ss)) !important; }
              #cv-preview .mt-1 { margin-top: calc(0.25rem * var(--ss)) !important; }
              #cv-preview .mt-1\\.5 { margin-top: calc(0.375rem * var(--ss)) !important; }
              #cv-preview .mt-2 { margin-top: calc(0.5rem * var(--ss)) !important; }
              #cv-preview .mt-3 { margin-top: calc(0.75rem * var(--ss)) !important; }
              
              #cv-preview .space-y-1 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(0.25rem * var(--ss)) !important; }
              #cv-preview .space-y-1\\.5 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(0.375rem * var(--ss)) !important; }
              #cv-preview .space-y-2 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(0.5rem * var(--ss)) !important; }
              #cv-preview .space-y-3 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(0.75rem * var(--ss)) !important; }
              #cv-preview .space-y-4 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(1rem * var(--ss)) !important; }
              #cv-preview .space-y-5 > :not([hidden]) ~ :not([hidden]) { margin-top: calc(1.25rem * var(--ss)) !important; }
              
              #cv-preview .gap-0\\.5 { gap: calc(0.125rem * var(--ss)) !important; }
              #cv-preview .gap-1 { gap: calc(0.25rem * var(--ss)) !important; }
              #cv-preview .gap-1\\.5 { gap: calc(0.375rem * var(--ss)) !important; }
              #cv-preview .gap-2 { gap: calc(0.5rem * var(--ss)) !important; }
              #cv-preview .gap-3 { gap: calc(0.75rem * var(--ss)) !important; }
              #cv-preview .gap-4 { gap: calc(1rem * var(--ss)) !important; }
              #cv-preview .gap-5 { gap: calc(1.25rem * var(--ss)) !important; }
              #cv-preview .gap-6 { gap: calc(1.5rem * var(--ss)) !important; }
            `}
          </style>
          <div className="max-w-[900px] mx-auto animate-scale-in print:max-w-none print:mx-0 print:animate-none">
            <div
              id="cv-preview"
              ref={previewRef}
              className="rounded-none overflow-hidden shadow-[0_8px_30px_rgb(0,0,0,0.12)] dark:shadow-[0_8px_30px_rgb(0,0,0,0.8)] ring-1 ring-black/5 dark:ring-white/5 transition-shadow duration-500 hover:shadow-[0_20px_60px_-15px_rgba(0,0,0,0.2)] dark:hover:shadow-[0_20px_60px_-15px_rgba(0,0,0,0.9)] print:shadow-none print:ring-0"
              style={{
                zoom: textScale,
                '--accent': accentColor,
                '--accent-light': accentColor + '20',
                '--sidebar': sidebarColor,
                fontFamily: `'${fontFamily}', sans-serif`,
              } as React.CSSProperties}
            >
              {activeTemplate === 'ats' ? (
                <ATSMinimalist />
              ) : activeTemplate === 'modern' ? (
                <ModernCreative />
              ) : activeTemplate === 'timeline' ? (
                <TimelineCreative />
              ) : (
                <TimelineTwoColumn />
              )}
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
