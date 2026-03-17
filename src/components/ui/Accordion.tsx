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
    <div className="rounded-2xl overflow-hidden bg-white/80 dark:bg-white/[0.04] backdrop-blur-lg border border-gray-200/60 dark:border-white/[0.06] shadow-sm shadow-gray-900/[0.04] dark:shadow-none hover:-translate-y-0.5 hover:shadow-md dark:hover:shadow-lg dark:hover:shadow-black/20 transition-all duration-200 animate-slide-up">
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between px-5 py-4 text-left group hover:bg-gray-50/60 dark:hover:bg-white/[0.03] active:scale-[0.995] transition-all duration-150"
      >
        <span className="flex items-center gap-2.5 font-semibold text-gray-800 dark:text-gray-100">
          {icon && (
            <span className={`text-lg transition-transform duration-300 ${isOpen ? 'scale-110' : 'scale-100'}`}>
              {icon}
            </span>
          )}
          {title}
        </span>
        <svg
          className={`w-4.5 h-4.5 text-gray-400 dark:text-gray-500 transition-transform duration-300 ease-[cubic-bezier(0.34,1.56,0.64,1)] ${isOpen ? 'rotate-180' : ''}`}
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
        <div className="px-5 pb-5 pt-1">{children}</div>
      </div>
    </div>
  );
}
