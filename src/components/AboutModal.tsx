'use client';

import { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';

const technologies = [
  { name: 'Next.js', description: 'React framework for the UI and routing' },
  { name: 'Vinext', description: 'Fast build tool and compiler wrapper' },
  { name: 'Tailwind CSS', description: 'Utility-first CSS framework for styling' },
  { name: 'Zustand', description: 'Lightweight state management with localStorage persistence' },
  { name: 'TypeScript', description: 'Static typing for robust code' },
  { name: 'PWA Ready', description: 'Installable as a Progressive Web App (Service Worker)' },
];

export default function AboutModal() {
  const [isOpen, setIsOpen] = useState(false);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  return (
    <>
      <button
        type="button"
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          setIsOpen(true);
        }}
        className="px-3.5 py-1.5 text-xs font-semibold rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/[0.06] active:scale-95 transition-all duration-150"
      >
        ℹ️ About
      </button>

      {mounted && isOpen && createPortal(
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-gray-900/50 backdrop-blur-sm animate-fade-in print:hidden">
          <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl w-full max-w-md max-h-[90vh] flex flex-col overflow-hidden animate-slide-up border border-gray-200 dark:border-gray-700">
            <div className="flex-shrink-0 flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
              <h2 className="text-lg font-bold text-gray-900 dark:text-white flex items-center gap-2">
                <div className="w-8 h-8 rounded-lg flex items-center justify-center overflow-hidden bg-white shadow-sm">
                  <img src="/cvapp_macos_logo.png" alt="CVapp Logo" className="w-full h-full object-contain" />
                </div>
                About CVapp
              </h2>
              <button
                onClick={() => setIsOpen(false)}
                className="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 transition-colors"
              >
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <div className="p-6 overflow-y-auto">
              <p className="text-sm text-gray-600 dark:text-gray-300 mb-6 leading-relaxed">
                CVapp is a fast, privacy-first resume builder. All data processing and PDF generation happens entirely in your local browser.
              </p>
              
              <h3 className="text-xs tracking-widest font-bold uppercase text-gray-400 dark:text-gray-500 mb-4">
                Technologies Used
              </h3>
              
              <ul className="space-y-4">
                {technologies.map((tech) => (
                  <li key={tech.name} className="flex flex-col">
                    <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                      {tech.name}
                    </span>
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {tech.description}
                    </span>
                  </li>
                ))}
              </ul>

              <div className="mt-6 pt-5 border-t border-gray-200 dark:border-gray-700">
                <h3 className="text-xs tracking-widest font-bold uppercase text-gray-400 dark:text-gray-500 mb-2">
                  License
                </h3>
                <p className="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">
                  Licensed under the{' '}
                  <a
                    href="https://www.gnu.org/licenses/gpl-3.0.en.html"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-indigo-600 dark:text-indigo-400 hover:underline font-medium"
                  >
                    GNU General Public License v3.0
                  </a>
                </p>
              </div>
            </div>
            
            <div className="flex-shrink-0 px-6 py-4 bg-gray-50 dark:bg-gray-800/50 border-t border-gray-200 dark:border-gray-700 flex justify-end">
              <button
                onClick={() => setIsOpen(false)}
                className="px-5 py-2 text-sm font-semibold text-white bg-indigo-600 hover:bg-indigo-700 rounded-lg shadow-sm transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>,
        document.body
      )}
    </>
  );
}
