'use client';

import { useResumeStore } from '@/store/useResumeStore';
import TagInput from '@/components/ui/TagInput';

export default function SkillsForm() {
  const skills = useResumeStore((s) => s.resumeData.skills);
  const setSkills = useResumeStore((s) => s.setSkills);

  return (
    <TagInput
      tags={skills}
      onChange={setSkills}
      label="Skills"
      placeholder="Type a skill and press Enter..."
    />
  );
}
