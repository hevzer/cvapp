import { useResumeStore } from '@/store/useResumeStore';

export default function SpacingSlider() {
  const spacingScale = useResumeStore((state) => state.spacingScale);
  const setSpacingScale = useResumeStore((state) => state.setSpacingScale);

  return (
    <div className="bg-white dark:bg-slate-900 p-4 rounded-2xl border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none space-y-3">
      <div className="flex items-center justify-between pointer-events-none">
        <label className="text-[11px] font-bold uppercase tracking-widest text-slate-700 dark:text-slate-300 flex items-center gap-2">
          <i className="bi bi-distribute-vertical text-[10px] text-cyan-500"></i>
          Content Spacing
        </label>
        <span className="text-[10px] font-bold tracking-wider bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 px-2 py-0.5 rounded-md">
          {Math.round(spacingScale * 100)}%
        </span>
      </div>
      <div className="flex items-center gap-3">
        <i className="bi bi-dash text-slate-400 text-xs"></i>
        <input
          type="range"
          min="0.1"
          max="2"
          step="0.05"
          value={spacingScale}
          onChange={(e) => setSpacingScale(parseFloat(e.target.value))}
          className="w-full h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
          aria-label="Adjust CV content spacing"
        />
        <i className="bi bi-plus text-slate-400 text-xs"></i>
      </div>
      <div className="flex justify-between text-[10px] text-slate-400 font-medium px-1">
        <span>Compact</span>
        <span>Standard</span>
        <span>Spacious</span>
      </div>
    </div>
  );
}
