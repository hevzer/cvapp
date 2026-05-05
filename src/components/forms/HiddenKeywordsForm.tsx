import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';

export default function HiddenKeywordsForm() {
  const hiddenKeywords = useResumeStore((s) => s.resumeData.hiddenKeywords);
  const setHiddenKeywords = useResumeStore((s) => s.setHiddenKeywords);

  return (
    <div className="space-y-3">
      <div className="flex items-start gap-2 p-3 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-800 rounded-lg">
        <span className="text-rose-600 text-lg mt-0.5">⚠️</span>
        <div className="text-xs text-rose-700 dark:text-rose-300 leading-relaxed space-y-1.5">
          <p className="font-bold">No longer rendered into your CV.</p>
          <p>
            Modern ATS (Greenhouse Harmonic, LLM-based screeners) detect zero-size hidden
            text and flag the resume as deceptive — the technique now hurts more than it
            helps. Templates produced by CVapp ship clean, well-structured HTML that
            scores higher on parser tests without it.
          </p>
          <p className="text-rose-600 dark:text-rose-400">
            Your existing keywords below are preserved in case you want to copy them into
            a real section, but they will not appear in the printed CV.
          </p>
        </div>
      </div>
      <TagInput
        tags={hiddenKeywords}
        onChange={setHiddenKeywords}
        placeholder="e.g., Kubernetes, Terraform, SDLC..."
      />
    </div>
  );
}
