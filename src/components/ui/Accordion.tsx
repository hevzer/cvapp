'use client';

import { useState, useRef, useEffect, type ReactNode } from 'react';

interface AccordionProps {
  title: string;
  icon?: string;
  defaultOpen?: boolean;
  children: ReactNode;
}

export default function Accordion({
  title,
  icon,
  defaultOpen = false,
  children,
}: AccordionProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);
  const contentRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(defaultOpen ? undefined : 0);

  useEffect(() => {
    if (!contentRef.current) return;
    if (isOpen) {
      const h = contentRef.current.scrollHeight;
      setHeight(h);
      const timer = setTimeout(() => setHeight(undefined), 300);
      return () => clearTimeout(timer);
    } else {
      const h = contentRef.current.scrollHeight;
      setHeight(h);
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          setHeight(0);
        });
      });
    }
  }, [isOpen]);

  return (
    <div className="rounded-2xl overflow-hidden bg-white dark:bg-slate-900 border border-transparent shadow-[0_2px_8px_rgb(0,0,0,0.08)] dark:shadow-none hover:shadow-[0_4px_16px_rgb(0,0,0,0.12)] transition-shadow duration-300">
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between px-6 py-4.5 text-left group hover:bg-slate-50 dark:hover:bg-slate-800/80 active:bg-slate-100 dark:active:bg-slate-800 transition-colors duration-200"
      >
        <span className="flex items-center gap-3 font-bold tracking-wide text-[15px] text-slate-800 dark:text-slate-100">
          {icon && (
            <span className={`text-xl transition-transform duration-300 ${isOpen ? 'scale-110 text-cyan-600 dark:text-cyan-400' : 'text-slate-500'}`}>
              {icon}
            </span>
          )}
          {title}
        </span>
        <svg
          className={`w-5 h-5 text-slate-400 transition-transform duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] ${isOpen ? 'rotate-180 text-cyan-500' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      <div
        ref={contentRef}
        className="overflow-hidden transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]"
        style={{
          height: height !== undefined ? `${height}px` : 'auto',
          opacity: isOpen ? 1 : 0,
        }}
      >
        <div className="px-6 pb-6 pt-2 border-t border-slate-100 dark:border-slate-800">{children}</div>
      </div>
    </div>
  );
}
