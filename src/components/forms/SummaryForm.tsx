'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function SummaryForm() {
  const summary = useResumeStore((s) => s.resumeData.summary);
  const updateSummary = useResumeStore((s) => s.updateSummary);

  return (
    <div>
      <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
        Professional Summary
      </label>
      <textarea
        value={summary}
        onChange={(e) => updateSummary(e.target.value)}
        placeholder="Write a brief summary of your professional background, key achievements, and career goals..."
        rows={5}
        className="w-full px-3 py-2 border border-gray-200 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100 placeholder-gray-400 text-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all resize-none"
      />
    </div>
  );
}
