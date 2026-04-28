import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';

export default function HiddenKeywordsForm() {
  const hiddenKeywords = useResumeStore((s) => s.resumeData.hiddenKeywords);
  const setHiddenKeywords = useResumeStore((s) => s.setHiddenKeywords);

  return (
    <div className="space-y-3">
      <div className="flex items-start gap-2 p-3 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-lg">
        <span className="text-amber-600 text-lg mt-0.5">⚡</span>
        <p className="text-xs text-amber-700 dark:text-amber-400 leading-relaxed">
          These keywords will be hidden in your PDF — invisible to human readers but
          detectable by ATS systems. Add job-specific skills and technologies from the
          job description to improve your match score.
        </p>
      </div>
      <TagInput
        tags={hiddenKeywords}
        onChange={setHiddenKeywords}
        placeholder="e.g., Kubernetes, Terraform, SDLC..."
      />
    </div>
  );
}
