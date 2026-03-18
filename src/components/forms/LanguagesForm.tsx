'use client';

import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';
import { i18n, getSafeLanguage } from '@/lib/i18n';

export default function LanguagesForm() {
  const languages = useResumeStore((s) => s.resumeData.languages);
  const setLanguages = useResumeStore((s) => s.setLanguages);
  const cvLanguage = getSafeLanguage(useResumeStore((s) => s.resumeData.cvLanguage));

  return (
    <TagInput
      tags={languages || []}
      onChange={setLanguages}
      label={i18n[cvLanguage].languages}
      placeholder="e.g. English (Native), Spanish (B2)..."
    />
  );
}
