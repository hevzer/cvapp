'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function FontScaleSlider() {
  const fontScale = useResumeStore((state) => state.fontScale);
  const setFontScale = useResumeStore((state) => state.setFontScale);

  return (
    <div className="bg-white dark:bg-slate-900 p-4 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-3">
      <div className="flex items-center justify-between pointer-events-none">
        <label className="text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
          <i className="bi bi-type text-[10px] text-cyan-500"></i>
          Font Size
        </label>
        <span className="text-[10px] font-bold tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-md">
          {Math.round(fontScale * 100)}%
        </span>
      </div>
      <div className="flex items-center gap-3">
        <i className="bi bi-type text-slate-400 text-[10px]"></i>
        <input
          type="range"
          min="0.5"
          max="1.5"
          step="0.05"
          value={fontScale}
          onChange={(e) => setFontScale(parseFloat(e.target.value))}
          className="w-full h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
          aria-label="Adjust CV font size"
        />
        <i className="bi bi-type text-slate-400 text-sm"></i>
      </div>
      <div className="flex justify-between text-[10px] text-slate-400 font-medium px-1">
        <span>Small</span>
        <span>Standard</span>
        <span>Large</span>
      </div>
    </div>
  );
}
