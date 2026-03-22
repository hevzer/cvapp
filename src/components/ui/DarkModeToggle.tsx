'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function DarkModeToggle() {
  const darkMode = useResumeStore((s) => s.darkMode);
  const toggleDarkMode = useResumeStore((s) => s.toggleDarkMode);

  return (
    <button
      type="button"
      onClick={toggleDarkMode}
      className={`relative flex items-center justify-center w-10 h-10 rounded-full transition-all duration-300 active:scale-90 outline-none ${
        darkMode 
          ? 'bg-slate-800 text-amber-300 hover:bg-slate-700 shadow-inner ring-1 ring-white/10' 
          : 'bg-white text-cyan-600 hover:bg-slate-50 shadow-sm border border-slate-200'
      }`}
      aria-label="Toggle dark mode"
    >
      <i className={`bi block transition-transform duration-500 ${darkMode ? 'bi-moon-stars-fill rotate-[360deg] scale-100' : 'bi-sun-fill rotate-0 scale-110'}`}></i>
    </button>
  );
}
