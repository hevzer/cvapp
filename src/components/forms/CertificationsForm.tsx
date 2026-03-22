'use client';

import { useResumeStore } from '@/store/useResumeStore';
import MaterialInput from '@/components/ui/MaterialInput';

export default function CertificationsForm() {
  const certifications = useResumeStore((s) => s.resumeData.certifications);
  const addCertification = useResumeStore((s) => s.addCertification);
  const removeCertification = useResumeStore((s) => s.removeCertification);
  const updateCertification = useResumeStore((s) => s.updateCertification);

  return (
    <div className="space-y-4">
      {certifications.map((cert, index) => (
        <div
          key={cert.id}
          className="p-5 border border-transparent shadow-md dark:border-slate-700/50 rounded-2xl bg-white dark:bg-slate-800/60 space-y-4 relative group"
        >
          <div className="flex items-center justify-between mb-2">
            <span className="text-[11px] font-bold tracking-widest uppercase text-slate-400 dark:text-slate-500">
              Certification #{index + 1}
            </span>
            <button
              type="button"
              onClick={() => removeCertification(cert.id)}
              className="text-[10px] tracking-wider uppercase text-red-500 hover:bg-red-50 dark:hover:bg-red-500/10 px-3 py-1.5 rounded-full font-bold transition-colors"
            >
              Remove
            </button>
          </div>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <MaterialInput
              type="text"
              label="Certification Name"
              value={cert.name}
              onChange={(e) => updateCertification(cert.id, { name: e.target.value })}
            />
            <MaterialInput
              type="text"
              label="Issuing Organization"
              value={cert.issuer}
              onChange={(e) => updateCertification(cert.id, { issuer: e.target.value })}
            />
            <MaterialInput
              type="month"
              label="Issue Date"
              value={cert.date}
              onChange={(e) => updateCertification(cert.id, { date: e.target.value })}
            />
            <MaterialInput
              type="text"
              label="Credential URL / ID"
              value={cert.url || ''}
              onChange={(e) => updateCertification(cert.id, { url: e.target.value })}
            />
          </div>
        </div>
      ))}
      
      <button
        type="button"
        onClick={addCertification}
        className="w-full py-3.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-full text-[13px] font-bold tracking-wider uppercase text-slate-500 hover:bg-slate-50 hover:text-cyan-600 hover:border-cyan-300 dark:hover:bg-slate-800 transition-colors"
      >
        + Add Certification
      </button>
    </div>
  );
}
