'use client';

import { useRef, useEffect, useState } from 'react';
import { useResumeStore } from '../store/useResumeStore';
import Accordion from './ui/Accordion';
import TemplateToggle from './ui/TemplateToggle';
import TextScaleSlider from './ui/TextScaleSlider';
import StyleCustomizer from './ui/StyleCustomizer';
import DarkModeToggle from './ui/DarkModeToggle';
import PersonalInfoForm from './forms/PersonalInfoForm';
import SummaryForm from './forms/SummaryForm';
import ExperienceForm from './forms/ExperienceForm';
import AboutModal from './AboutModal';
import EducationForm from './forms/EducationForm';
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
          <div className="w-14 h-14 rounded-2xl bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 flex items-center justify-center shadow-xl shadow-indigo-500/20 animate-pulse">
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
    <div className="flex flex-col h-screen bg-gradient-to-br from-slate-100 via-gray-50 to-slate-100 dark:from-gray-900 dark:via-gray-900 dark:to-gray-950 print:bg-none print:h-auto print:block">
      {/* ── Header ── */}
      <header className="flex-shrink-0 bg-white/70 dark:bg-gray-900/70 backdrop-blur-2xl border-b border-gray-200/60 dark:border-white/[0.06] px-6 py-3 animate-slide-up print:hidden">
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
              className="px-3.5 py-1.5 text-xs font-semibold rounded-lg text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-500/10 active:scale-95 transition-all duration-150"
            >
              ✨ Fill Example
            </button>
            <button
              type="button"
              onClick={clearAllData}
              className="px-3.5 py-1.5 text-xs font-semibold rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/[0.06] active:scale-95 transition-all duration-150"
            >
              Clear All
            </button>
            <div className="w-px h-5 bg-gray-200 dark:bg-white/10 mx-1.5" />
            <select
              value={cvLang}
              onChange={(e) => setCvLang(e.target.value)}
              className="text-xs font-medium border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg px-2 py-1 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all hover:border-gray-300 dark:hover:border-gray-500 cursor-pointer"
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
        <aside className="w-[440px] flex-shrink-0 overflow-y-auto border-r border-gray-200/60 dark:border-white/[0.06] bg-white/40 dark:bg-gray-900/40 backdrop-blur-xl p-5 animate-stagger space-y-4 print:hidden">
          <TemplateToggle />
          <TextScaleSlider />
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
        <main className="flex-1 overflow-y-auto bg-gradient-to-br from-gray-100 via-gray-200 to-gray-100 dark:from-gray-950 dark:via-gray-900 dark:to-gray-950 p-8 print:p-0 print:m-0 print:bg-none print:overflow-visible">
          <div className="max-w-[900px] mx-auto animate-scale-in print:max-w-none print:mx-0 print:animate-none">
            <div
              id="cv-preview"
              ref={previewRef}
              className="rounded-2xl overflow-hidden shadow-2xl ring-1 ring-black/5 dark:ring-white/5 transition-shadow duration-500 hover:shadow-[0_30px_80px_-20px_rgba(0,0,0,0.15)] dark:hover:shadow-[0_30px_80px_-20px_rgba(0,0,0,0.6)] print:shadow-none print:ring-0 print:rounded-none"
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
