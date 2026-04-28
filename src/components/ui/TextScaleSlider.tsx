import { useResumeStore } from '@/store/useResumeStore';

export default function TextScaleSlider() {
  const textScale = useResumeStore((s) => s.textScale);
  const setTextScale = useResumeStore((s) => s.setTextScale);

  const pct = Math.round(textScale * 100);

  return (
    <div className="flex items-center gap-3 px-1 pt-2">
      <i className="bi bi-type text-slate-400 dark:text-slate-500 text-sm"></i>
      <input
        type="range"
        min={80}
        max={120}
        step={5}
        value={pct}
        onChange={(e) => setTextScale(Number(e.target.value) / 100)}
        className="flex-1 h-2 bg-slate-200 dark:bg-slate-700/50 rounded-full appearance-none cursor-pointer accent-cyan-600 [&::-webkit-slider-thumb]:w-5 [&::-webkit-slider-thumb]:h-5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-cyan-600 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-lg hover:[&::-webkit-slider-thumb]:scale-110 [&::-webkit-slider-thumb]:transition-transform"
      />
      <span className="text-xs font-bold text-slate-500 dark:text-slate-400 min-w-[36px] text-right tabular-nums">
        {pct}%
      </span>
      {pct !== 100 && (
        <button
          type="button"
          onClick={() => setTextScale(1)}
          className="text-[10px] text-slate-400 hover:text-cyan-600 transition-colors"
          title="Reset to 100%"
        >
          <i className="bi bi-arrow-counterclockwise"></i>
        </button>
      )}
    </div>
  );
}
