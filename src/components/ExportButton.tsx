'use client';

import { useState } from 'react';

interface ExportButtonProps {
  targetRef: React.RefObject<HTMLDivElement | null>;
  filename?: string;
}

export default function ExportButton({ targetRef, filename = 'cv' }: ExportButtonProps) {
  const [loading, setLoading] = useState(false);

  const handleExport = () => {
    window.print();
  };

  return (
    <button
      type="button"
      onClick={handleExport}
      disabled={loading}
      className="w-full flex items-center justify-center gap-2.5 px-5 py-3.5 rounded-2xl text-sm font-bold text-white bg-gradient-to-r from-red-600 via-rose-600 to-red-600 bg-[length:200%_auto] hover:bg-right shadow-lg shadow-red-500/25 hover:shadow-xl hover:shadow-red-500/30 active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed transition-all duration-300"
    >
      {loading ? (
        <>
          <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.37 0 0 5.37 0 12h4z" />
          </svg>
          Generating PDF…
        </>
      ) : (
        <>
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          Export to PDF
        </>
      )}
    </button>
  );
}
