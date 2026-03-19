'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function SpacingSlider() {
  const spacingScale = useResumeStore((state) => state.spacingScale);
  const setSpacingScale = useResumeStore((state) => state.setSpacingScale);

  return (
    <div className="bg-white/80 dark:bg-slate-900/80 p-3 rounded-lg border border-slate-200/50 dark:border-slate-700/50 backdrop-blur-sm shadow-sm space-y-2">
      <div className="flex items-center justify-between">
        <label className="text-xs font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
          <i className="bi bi-distribute-vertical text-[10px]"></i>
          Content Spacing
        </label>
        <span className="text-[10px] font-medium bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-full">
          {Math.round(spacingScale * 100)}%
        </span>
      </div>
      <div className="flex items-center gap-2">
        <i className="bi bi-dash text-slate-400 text-xs"></i>
        <input
          type="range"
          min="0.5"
          max="1.5"
          step="0.05"
          value={spacingScale}
          onChange={(e) => setSpacingScale(parseFloat(e.target.value))}
          className="w-full h-1.5 bg-slate-200 dark:bg-slate-700 rounded-lg appearance-none cursor-pointer accent-indigo-500"
          aria-label="Adjust CV content spacing"
        />
        <i className="bi bi-plus text-slate-400 text-xs"></i>
      </div>
      <div className="flex justify-between text-[9px] text-slate-400 font-medium px-1">
        <span>Compact</span>
        <span>Standard</span>
        <span>Spacious</span>
      </div>
    </div>
  );
}
