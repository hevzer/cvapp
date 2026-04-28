import { useResumeStore } from '@/store/useResumeStore';
import MaterialInput from '@/components/ui/MaterialInput';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function EducationForm() {
  const education = useResumeStore((s) => s.resumeData.education);
  const addEducation = useResumeStore((s) => s.addEducation);
  const removeEducation = useResumeStore((s) => s.removeEducation);
  const updateEducation = useResumeStore((s) => s.updateEducation);

  return (
    <div className="space-y-4">
      {education.map((edu, index) => (
        <div
          key={edu.id}
          className="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group"
        >
          <div className="flex items-center justify-between mb-2">
            <span className="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
              Education #{index + 1}
            </span>
            <button
              type="button"
              onClick={() => removeEducation(edu.id)}
              className="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
            >
              Remove
            </button>
          </div>
          
          <MaterialInput
            type="text"
            label="Institution (University / School)"
            value={edu.institution}
            onChange={(e) => updateEducation(edu.id, { institution: e.target.value })}
          />
          
          <div className="grid grid-cols-2 gap-3">
            <MaterialInput
              type="text"
              label="Degree (e.g. B.S.)"
              value={edu.degree}
              onChange={(e) => updateEducation(edu.id, { degree: e.target.value })}
            />
            <MaterialInput
              type="text"
              label="Field of Study"
              value={edu.field}
              onChange={(e) => updateEducation(edu.id, { field: e.target.value })}
            />
          </div>
          
          <div className="grid grid-cols-2 gap-3">
            <MaterialInput
              type="month"
              label="Start Date"
              value={edu.startDate}
              onChange={(e) => updateEducation(edu.id, { startDate: e.target.value })}
            />
            <MaterialInput
              type="month"
              label="End Date"
              value={edu.endDate}
              onChange={(e) => updateEducation(edu.id, { endDate: e.target.value })}
            />
          </div>
          
          <MaterialTextarea
            label="Description & Coursework"
            value={edu.description}
            onChange={(e) => updateEducation(edu.id, { description: e.target.value })}
            rows={3}
          />
        </div>
      ))}
      
      <button
        type="button"
        onClick={addEducation}
        className="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
      >
        + Add Education
      </button>
    </div>
  );
}
