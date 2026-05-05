import { useResumeStore } from '@/store/useResumeStore';

const colorPresets = [
  { name: 'Indigo', hex: '#6366f1' },
  { name: 'Blue', hex: '#3b82f6' },
  { name: 'Emerald', hex: '#10b981' },
  { name: 'Rose', hex: '#f43f5e' },
  { name: 'Amber', hex: '#f59e0b' },
  { name: 'Cyan', hex: '#06b6d4' },
  { name: 'Slate', hex: '#475569' },
  { name: 'Teal', hex: '#14b8a6' },
];

const sidebarPresets = [
  { name: 'Slate 800', hex: '#1e293b' },
  { name: 'Zinc 900', hex: '#18181b' },
  { name: 'Navy', hex: '#0f172a' },
  { name: 'Midnight', hex: '#020617' },
  { name: 'Emerald 900', hex: '#064e3b' },
  { name: 'Rose 900', hex: '#4c0519' },
  { name: 'Cyan 900', hex: '#164e63' },
  { name: 'Black', hex: '#000000' },
];

const fontOptions = [
  { name: 'Inter', style: "'Inter', sans-serif" },
  { name: 'Roboto', style: "'Roboto', sans-serif" },
  { name: 'Outfit', style: "'Outfit', sans-serif" },
  { name: 'Lora', style: "'Lora', serif" },
  { name: 'Merriweather', style: "'Merriweather', serif" },
];

export default function StyleCustomizer() {
  const accentColor = useResumeStore((s) => s.accentColor);
  const setAccentColor = useResumeStore((s) => s.setAccentColor);
  const sidebarColor = useResumeStore((s) => s.sidebarColor);
  const setSidebarColor = useResumeStore((s) => s.setSidebarColor);
  const fontFamily = useResumeStore((s) => s.fontFamily);
  const setFontFamily = useResumeStore((s) => s.setFontFamily);
  const hidePhoto = useResumeStore((s) => s.hidePhoto);
  const setHidePhoto = useResumeStore((s) => s.setHidePhoto);

  return (
    <div className="bg-white dark:bg-slate-900 p-5 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-5">
      {/* Accent Color */}
      <div>
        <label className="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-3 block">
          <i className="bi bi-palette-fill text-[11px] text-cyan-500"></i>
          Accent Color
        </label>
        <div className="flex items-center gap-2 flex-wrap">
          {colorPresets.map((color) => (
            <button
              key={color.hex}
              type="button"
              title={color.name}
              onClick={() => setAccentColor(color.hex)}
              className="relative w-7 h-7 flex items-center justify-center rounded-full transition-all duration-300 active:scale-90 outline-none hover:shadow-md"
              style={{ backgroundColor: color.hex }}
            >
              <i className={`bi bi-check2 text-white text-sm transition-transform duration-300 ${accentColor === color.hex ? 'scale-100 opacity-100' : 'scale-0 opacity-0'}`}></i>
            </button>
          ))}
        </div>
      </div>

      {/* Sidebar Color */}
      <div>
        <label className="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-3 block">
          <i className="bi bi-layout-sidebar-inset text-[11px] text-cyan-500"></i>
          Sidebar Color
        </label>
        <div className="flex items-center gap-2 flex-wrap">
          {sidebarPresets.map((color) => (
            <button
              key={color.hex}
              type="button"
              title={color.name}
              onClick={() => setSidebarColor(color.hex)}
              className="relative w-7 h-7 flex items-center justify-center rounded-full transition-all duration-300 active:scale-90 outline-none hover:shadow-[0_2px_8px_rgb(0,0,0,0.3)] border border-white/10"
              style={{ backgroundColor: color.hex }}
            >
              <i className={`bi bi-check2 text-white text-sm transition-transform duration-300 ${sidebarColor === color.hex ? 'scale-100 opacity-100' : 'scale-0 opacity-0'}`}></i>
            </button>
          ))}
        </div>
      </div>

      {/* Font Family */}
      <div>
        <label className="flex items-center gap-2 text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 mb-2 block">
          <i className="bi bi-fonts text-[12px] text-cyan-500"></i>
          Typography
        </label>
        <select
          value={fontFamily}
          onChange={(e) => setFontFamily(e.target.value)}
          className="w-full text-[13px] font-bold uppercase tracking-widest text-slate-800 dark:text-slate-200 bg-slate-50 dark:bg-slate-800 py-3 px-4 rounded-xl border border-slate-200 dark:border-slate-700 outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 appearance-none shadow-sm cursor-pointer transition-all hover:bg-slate-100 dark:hover:bg-slate-700/80"
          style={{ backgroundImage: `url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e")`, backgroundPosition: 'right 0.5rem center', backgroundRepeat: 'no-repeat', backgroundSize: '1.5em 1.5em' }}
        >
          {fontOptions.map((font) => (
            <option key={font.name} value={font.name} style={{ fontFamily: font.style }} className="font-semibold text-sm capitalize tracking-normal">
              {font.name}
            </option>
          ))}
        </select>
      </div>

      {/* Hide Photo (ATS-safe for US/UK) */}
      <div>
        <label className="flex items-start gap-3 cursor-pointer select-none">
          <input
            type="checkbox"
            checked={hidePhoto}
            onChange={(e) => setHidePhoto(e.target.checked)}
            className="mt-0.5 rounded border-slate-300 text-cyan-600 focus:ring-cyan-500 w-4 h-4 cursor-pointer"
          />
          <span className="flex-1">
            <span className="block text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300">
              Hide photo
            </span>
            <span className="block text-[11px] text-slate-500 dark:text-slate-400 mt-0.5 leading-snug">
              Recommended for US/UK applications. Some ATS flag photos as bias risk.
            </span>
          </span>
        </label>
      </div>
    </div>
  );
}
