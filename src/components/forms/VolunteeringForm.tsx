import { useResumeStore } from '@/store/useResumeStore';
import MaterialInput from '@/components/ui/MaterialInput';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function VolunteeringForm() {
  const volunteering = useResumeStore((s) => s.resumeData.volunteering ?? []);
  const addVolunteering = useResumeStore((s) => s.addVolunteering);
  const removeVolunteering = useResumeStore((s) => s.removeVolunteering);
  const updateVolunteering = useResumeStore((s) => s.updateVolunteering);

  return (
    <div className="space-y-4">
      {volunteering.map((entry, index) => (
        <div
          key={entry.id}
          className="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group"
        >
          <div className="flex items-center justify-between mb-2">
            <span className="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
              Volunteering #{index + 1}
            </span>
            <button
              type="button"
              onClick={() => removeVolunteering(entry.id)}
              className="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
            >
              Remove
            </button>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <MaterialInput
              type="text"
              label="Organization"
              value={entry.organization}
              onChange={(e) => updateVolunteering(entry.id, { organization: e.target.value })}
            />
            <MaterialInput
              type="text"
              label="Role"
              value={entry.role}
              onChange={(e) => updateVolunteering(entry.id, { role: e.target.value })}
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <MaterialInput
              type="month"
              label="Start Date"
              value={entry.startDate}
              onChange={(e) => updateVolunteering(entry.id, { startDate: e.target.value })}
            />
            <MaterialInput
              type="month"
              label="End Date"
              value={entry.endDate}
              onChange={(e) => updateVolunteering(entry.id, { endDate: e.target.value })}
              disabled={entry.current}
            />
          </div>

          <label className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-300 cursor-pointer pt-1 pb-2">
            <input
              type="checkbox"
              checked={entry.current}
              onChange={(e) =>
                updateVolunteering(entry.id, { current: e.target.checked, endDate: '' })
              }
              className="rounded border-slate-300 text-cyan-600 focus:ring-cyan-500 w-4 h-4 cursor-pointer"
            />
            <span className="font-medium text-[13px]">Currently volunteering</span>
          </label>

          <MaterialTextarea
            label="Description & Impact"
            value={entry.description}
            onChange={(e) => updateVolunteering(entry.id, { description: e.target.value })}
            rows={4}
          />
        </div>
      ))}

      <button
        type="button"
        onClick={addVolunteering}
        className="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
      >
        + Add Volunteering
      </button>
    </div>
  );
}
