'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function DarkModeToggle() {
  const darkMode = useResumeStore((s) => s.darkMode);
  const toggleDarkMode = useResumeStore((s) => s.toggleDarkMode);

  return (
    <button
      type="button"
      onClick={toggleDarkMode}
      className="group relative flex items-center w-16 h-8 rounded-full p-1 transition-all duration-500 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-800"
      style={{
        background: darkMode
          ? 'linear-gradient(135deg, #1e1b4b 0%, #312e81 50%, #1e1b4b 100%)'
          : 'linear-gradient(135deg, #93c5fd 0%, #60a5fa 50%, #38bdf8 100%)',
      }}
      aria-label="Toggle dark mode"
    >
      {/* Stars (dark mode) */}
      <span
        className={`absolute top-1.5 right-3 w-1 h-1 rounded-full bg-white transition-all duration-500 ${
          darkMode ? 'opacity-100 scale-100' : 'opacity-0 scale-0'
        }`}
      />
      <span
        className={`absolute top-3 right-5 w-0.5 h-0.5 rounded-full bg-white/70 transition-all duration-700 ${
          darkMode ? 'opacity-100 scale-100' : 'opacity-0 scale-0'
        }`}
      />
      <span
        className={`absolute bottom-2 right-2.5 w-0.5 h-0.5 rounded-full bg-white/50 transition-all duration-600 ${
          darkMode ? 'opacity-100 scale-100' : 'opacity-0 scale-0'
        }`}
      />

      {/* Cloud (light mode) */}
      <span
        className={`absolute top-1 left-8 w-3 h-1.5 rounded-full bg-white/40 transition-all duration-500 ${
          darkMode ? 'opacity-0 translate-x-2' : 'opacity-100 translate-x-0'
        }`}
      />
      <span
        className={`absolute bottom-1.5 left-7 w-4 h-1.5 rounded-full bg-white/30 transition-all duration-700 ${
          darkMode ? 'opacity-0 translate-x-3' : 'opacity-100 translate-x-0'
        }`}
      />

      {/* Toggle Thumb */}
      <span
        className={`relative z-10 flex items-center justify-center w-6 h-6 rounded-full shadow-lg transition-all duration-500 ease-[cubic-bezier(0.68,-0.55,0.27,1.55)] ${
          darkMode
            ? 'translate-x-8 bg-gray-200'
            : 'translate-x-0 bg-yellow-300'
        }`}
      >
        {/* Sun Icon */}
        <svg
          className={`absolute w-4 h-4 text-amber-500 transition-all duration-500 ${
            darkMode ? 'opacity-0 rotate-90 scale-0' : 'opacity-100 rotate-0 scale-100'
          }`}
          fill="currentColor"
          viewBox="0 0 20 20"
        >
          <path
            fillRule="evenodd"
            d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
            clipRule="evenodd"
          />
        </svg>

        {/* Moon Icon */}
        <svg
          className={`absolute w-3.5 h-3.5 text-indigo-700 transition-all duration-500 ${
            darkMode ? 'opacity-100 rotate-0 scale-100' : 'opacity-0 -rotate-90 scale-0'
          }`}
          fill="currentColor"
          viewBox="0 0 20 20"
        >
          <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
        </svg>
      </span>
    </button>
  );
}
