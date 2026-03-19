'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function FontScaleSlider() {
  const fontScale = useResumeStore((state) => state.fontScale);
  const setFontScale = useResumeStore((state) => state.setFontScale);

  return (
    <div className="bg-white/80 dark:bg-slate-900/80 p-3 rounded-lg border border-slate-200/50 dark:border-slate-700/50 backdrop-blur-sm shadow-sm space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-xs font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
          <i className="bi bi-type text-[10px]"></i>
          Font Size
        </label>
        <span className="text-[10px] font-medium bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-full">
          {Math.round(fontScale * 100)}%
        </span>
      </div>
      <div className="flex items-center gap-2">
        <i className="bi bi-type text-slate-400 text-[10px]"></i>
        <input
          type="range"
          min="0.8"
          max="1.2"
          step="0.02"
          value={fontScale}
          onChange={(e) => setFontScale(parseFloat(e.target.value))}
          className="w-full h-1.5 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-indigo-500"
          aria-label="Adjust CV font size"
        />
        <i className="bi bi-type text-slate-400 text-sm"></i>
      </div>
      <div className="flex justify-between text-[9px] text-slate-400 font-medium px-1">
        <span>Small</span>
        <span>Standard</span>
        <span>Large</span>
      </div>
    </div>
  );
}
