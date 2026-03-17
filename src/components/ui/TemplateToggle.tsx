'use client';

import { useResumeStore } from '@/store/useResumeStore';
import type { TemplateType } from '@/types/resume';

const templates: { id: TemplateType; label: string; desc: string }[] = [
  { id: 'ats', label: 'ATS Minimalist', desc: 'Clean, single-column' },
  { id: 'modern', label: 'Modern Creative', desc: 'Two-column, styled' },
  { id: 'timeline', label: 'Timeline', desc: 'Vertical experience flow' },
  { id: 'timelineTwoColumn', label: 'Timeline Two-Column', desc: 'Timeline with sidebar' },
];

export default function TemplateToggle() {
  const activeTemplate = useResumeStore((s) => s.activeTemplate);
  const setActiveTemplate = useResumeStore((s) => s.setActiveTemplate);

  return (
    <div className="flex gap-2">
      {templates.map((t) => (
        <button
          key={t.id}
          type="button"
          onClick={() => setActiveTemplate(t.id)}
          className={`flex-1 px-4 py-3 rounded-2xl text-sm font-medium border transition-all duration-200 active:scale-[0.98] ${
            activeTemplate === t.id
              ? 'border-indigo-500/50 bg-indigo-50 dark:bg-indigo-500/10 text-indigo-700 dark:text-indigo-300 shadow-sm shadow-indigo-500/10 ring-1 ring-indigo-500/30'
              : 'border-gray-200/60 dark:border-white/[0.06] bg-white/60 dark:bg-white/[0.03] text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:hover:border-white/10 hover:bg-gray-50 dark:hover:bg-white/[0.05]'
          }`}
        >
          <div className="font-semibold">{t.label}</div>
          <div className="text-xs opacity-60 mt-0.5">{t.desc}</div>
        </button>
      ))}
    </div>
  );
}
