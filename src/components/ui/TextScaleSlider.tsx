'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function TextScaleSlider() {
  const textScale = useResumeStore((s) => s.textScale);
  const setTextScale = useResumeStore((s) => s.setTextScale);

  const pct = Math.round(textScale * 100);

  return (
    <div className="flex items-center gap-3 px-1">
      <i className="bi bi-type text-gray-400 dark:text-gray-500 text-sm"></i>
      <input
        type="range"
        min={80}
        max={120}
        step={5}
        value={pct}
        onChange={(e) => setTextScale(Number(e.target.value) / 100)}
        className="flex-1 h-1.5 bg-gray-200 dark:bg-gray-700 rounded-full appearance-none cursor-pointer accent-indigo-500 [&::-webkit-slider-thumb]:w-3.5 [&::-webkit-slider-thumb]:h-3.5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:shadow-md"
      />
      <span className="text-xs font-semibold text-gray-500 dark:text-gray-400 min-w-[36px] text-right tabular-nums">
        {pct}%
      </span>
      {pct !== 100 && (
        <button
          type="button"
          onClick={() => setTextScale(1)}
          className="text-[10px] text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          title="Reset to 100%"
        >
          <i className="bi bi-arrow-counterclockwise"></i>
        </button>
      )}
    </div>
  );
}
