'use client';

import { useResumeStore } from '@/store/useResumeStore';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function SummaryForm() {
  const summary = useResumeStore((s) => s.resumeData.summary);
  const updateSummary = useResumeStore((s) => s.updateSummary);

  return (
    <div className="pt-2">
      <MaterialTextarea
        label="Professional Summary"
        value={summary || ''}
        onChange={(e) => updateSummary(e.target.value)}
        rows={6}
      />
    </div>
  );
}
