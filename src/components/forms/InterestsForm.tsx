import { useResumeStore } from '@/store/useResumeStore';
import type { Interest } from '@/types/resume';
import MaterialInput from '@/components/ui/MaterialInput';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function InterestsForm() {
  const interests = useResumeStore((s) => s.resumeData.interests ?? []);
  const addInterest = useResumeStore((s) => s.addInterest);
  const removeInterest = useResumeStore((s) => s.removeInterest);
  const updateInterest = useResumeStore((s) => s.updateInterest);

  // Migration to objects happens at the store boundary; safe to cast here.
  const items = interests as Interest[];

  return (
    <div className="space-y-4">
      {items.map((entry, index) => (
        <div
          key={entry.id}
          className="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group"
        >
          <div className="flex items-center justify-between mb-2">
            <span className="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
              Interest #{index + 1}
            </span>
            <button
              type="button"
              onClick={() => removeInterest(entry.id)}
              className="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
            >
              Remove
            </button>
          </div>

          <MaterialInput
            type="text"
            label="Name (e.g. Photography)"
            value={entry.name}
            onChange={(e) => updateInterest(entry.id, { name: e.target.value })}
          />

          <MaterialTextarea
            label="Description (Optional)"
            value={entry.description}
            onChange={(e) => updateInterest(entry.id, { description: e.target.value })}
            rows={2}
          />
        </div>
      ))}

      <button
        type="button"
        onClick={addInterest}
        className="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
      >
        + Add Interest
      </button>
    </div>
  );
}
