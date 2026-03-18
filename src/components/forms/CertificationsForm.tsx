'use client';

import { useResumeStore } from '@/store/useResumeStore';

export default function CertificationsForm() {
  const certifications = useResumeStore((s) => s.resumeData.certifications);
  const addCertification = useResumeStore((s) => s.addCertification);
  const removeCertification = useResumeStore((s) => s.removeCertification);
  const updateCertification = useResumeStore((s) => s.updateCertification);

  return (
    <div className="space-y-4">
      {certifications.map((cert, index) => (
        <div key={cert.id} className="p-4 border border-slate-200 dark:border-slate-700 rounded-lg bg-slate-50 dark:bg-slate-800/50 space-y-4 relative">
          <div className="flex justify-between items-center pb-2 border-b border-slate-200 dark:border-slate-700">
            <h3 className="font-medium text-slate-800 dark:text-slate-200 text-sm">Certification {index + 1}</h3>
            <button
              type="button"
              onClick={() => removeCertification(cert.id)}
              className="text-red-500 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300 transition-colors p-1"
              aria-label="Remove certification"
            >
              <i className="bi bi-trash3"></i>
            </button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Certification Name</label>
              <input
                type="text"
                value={cert.name}
                onChange={(e) => updateCertification(cert.id, { name: e.target.value })}
                className="w-full p-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-900 text-slate-900 dark:text-white"
                placeholder="e.g. AWS Certified Developer"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Issuing Organization</label>
              <input
                type="text"
                value={cert.issuer}
                onChange={(e) => updateCertification(cert.id, { issuer: e.target.value })}
                className="w-full p-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-900 text-slate-900 dark:text-white"
                placeholder="e.g. Amazon Web Services"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Issue Date</label>
              <input
                type="month"
                value={cert.date}
                onChange={(e) => updateCertification(cert.id, { date: e.target.value })}
                className="w-full p-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-900 text-slate-900 dark:text-white [color-scheme:light] dark:[color-scheme:dark]"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Credential URL / ID</label>
              <input
                type="text"
                value={cert.url || ''}
                onChange={(e) => updateCertification(cert.id, { url: e.target.value })}
                className="w-full p-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-900 text-slate-900 dark:text-white"
                placeholder="e.g. https://www.credly.com/badges/..."
              />
            </div>
          </div>
        </div>
      ))}

      <button
        type="button"
        onClick={addCertification}
        className="w-full py-2.5 border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-lg text-slate-600 dark:text-slate-400 hover:border-slate-400 dark:hover:border-slate-500 hover:text-slate-800 dark:hover:text-slate-200 transition-colors font-medium flex items-center justify-center gap-2"
      >
        <i className="bi bi-plus-circle"></i> Add Certification
      </button>
    </div>
  );
}
