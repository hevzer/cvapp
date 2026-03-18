'use client';

import { useResumeStore } from '@/store/useResumeStore';

const colorPresets = [
  { name: 'Indigo', hex: '#6366f1' },
  { name: 'Blue', hex: '#3b82f6' },
  { name: 'Emerald', hex: '#10b981' },
  { name: 'Rose', hex: '#f43f5e' },
  { name: 'Amber', hex: '#f59e0b' },
  { name: 'Violet', hex: '#8b5cf6' },
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
  { name: 'Violet 900', hex: '#2e1065' },
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

  return (
    <div className="space-y-3">
      {/* Accent Color */}
      <div>
        <label className="text-[11px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-2 block">
          Accent Color
        </label>
        <div className="flex items-center gap-1.5 flex-wrap">
          {colorPresets.map((color) => (
            <button
              key={color.hex}
              type="button"
              title={color.name}
              onClick={() => setAccentColor(color.hex)}
              className={`w-6 h-6 rounded-full transition-all duration-150 hover:scale-110 active:scale-95 ring-offset-2 ring-offset-white dark:ring-offset-gray-900 ${
                accentColor === color.hex
                  ? 'ring-2 ring-gray-400 dark:ring-gray-300 scale-110'
                  : 'ring-1 ring-black/10 dark:ring-white/10'
              }`}
              style={{ backgroundColor: color.hex }}
            />
          ))}
        </div>
      </div>

      {/* Sidebar Color */}
      <div>
        <label className="text-[11px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-2 block">
          Sidebar Color
        </label>
        <div className="flex items-center gap-1.5 flex-wrap">
          {sidebarPresets.map((color) => (
            <button
              key={color.hex}
              type="button"
              title={color.name}
              onClick={() => setSidebarColor(color.hex)}
              className={`w-6 h-6 rounded-full transition-all duration-150 hover:scale-110 active:scale-95 ring-offset-2 ring-offset-white dark:ring-offset-gray-900 ${
                sidebarColor === color.hex
                  ? 'ring-2 ring-gray-400 dark:ring-gray-300 scale-110'
                  : 'ring-1 ring-black/10 dark:ring-white/10'
              }`}
              style={{ backgroundColor: color.hex }}
            />
          ))}
        </div>
      </div>

      {/* Font Family */}
      <div>
        <label className="text-[11px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-1.5 block">
          Font
        </label>
        <select
          value={fontFamily}
          onChange={(e) => setFontFamily(e.target.value)}
          className="w-full text-sm font-medium border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg px-3 py-1.5 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all hover:border-gray-300 dark:hover:border-gray-500 cursor-pointer"
        >
          {fontOptions.map((font) => (
            <option key={font.name} value={font.name} style={{ fontFamily: font.style }}>
              {font.name}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
