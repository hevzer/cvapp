import { useResumeStore } from '@/store/useResumeStore';
import type { TemplateType } from '@/types/resume';

const templates: { id: TemplateType; label: string; desc: string; icon: string }[] = [
  { id: 'ats', label: 'ATS Minimalist', desc: 'Strict semantic flow', icon: 'file-text' },
  { id: 'modern', label: 'Modern Creative', desc: 'Sleek two-column', icon: 'columns-gap' },
  { id: 'timeline', label: 'Timeline', desc: 'Vertical chronographics', icon: 'clock-history' },
  { id: 'timelineTwoColumn', label: 'Split Timeline', desc: 'Sidebar & Timeline', icon: 'layout-sidebar' },
];

export default function TemplateToggle() {
  const activeTemplate = useResumeStore((s) => s.activeTemplate);
  const setActiveTemplate = useResumeStore((s) => s.setActiveTemplate);

  return (
    <div className="grid grid-cols-2 gap-3 mb-1">
      {templates.map((t) => (
        <button
          key={t.id}
          type="button"
          onClick={() => setActiveTemplate(t.id)}
          className={`relative flex flex-col items-start p-4 rounded-2xl text-left border transition-all duration-300 outline-none active:scale-[0.98] ${
            activeTemplate === t.id
              ? 'bg-cyan-50 border-cyan-200 dark:bg-cyan-900/40 dark:border-cyan-500/50 shadow-sm'
              : 'bg-white border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.06)] hover:shadow-md dark:shadow-none dark:bg-slate-800 dark:hover:bg-slate-800/80 hover:border-slate-200 dark:hover:border-slate-700'
          }`}
        >
          <div className={`mb-2 w-8 h-8 rounded-full flex items-center justify-center transition-colors ${
            activeTemplate === t.id ? 'bg-cyan-600 text-white' : 'bg-slate-100 dark:bg-slate-700 text-slate-500 dark:text-slate-400'
          }`}>
            <i className={`bi bi-${t.icon} text-[13px]`}></i>
          </div>
          <div className={`text-[12px] font-bold tracking-wide uppercase transition-colors ${
            activeTemplate === t.id ? 'text-cyan-900 dark:text-cyan-200' : 'text-slate-700 dark:text-slate-200'
          }`}>
            {t.label}
          </div>
          <div className={`text-[10px] mt-1 font-medium transition-colors ${
            activeTemplate === t.id ? 'text-cyan-700/80 dark:text-cyan-300/80' : 'text-slate-500 dark:text-slate-400'
          }`}>
            {t.desc}
          </div>
          
          {/* Active Radio Indicator */}
          {activeTemplate === t.id && (
            <div className="absolute top-4 right-4 w-4 h-4 bg-cyan-600 rounded-full flex items-center justify-center shadow-sm">
              <i className="bi bi-check2 text-white text-[10px]"></i>
            </div>
          )}
        </button>
      ))}
    </div>
  );
}
