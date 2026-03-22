'use client';

import { useRef, useState, useEffect } from 'react';
import { useResumeStore } from '../../store/useResumeStore';

export default function DataActions() {
  const resumeData = useResumeStore((s) => s.resumeData);
  const setResumeData = useResumeStore((s) => s.setResumeData);
  const fileRef = useRef<HTMLInputElement>(null);
  
  const [fileHandle, setFileHandle] = useState<any>(null); // FileSystemFileHandle
  const [isSupported, setIsSupported] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle');

  useEffect(() => {
    setIsSupported(typeof window !== 'undefined' && 'showOpenFilePicker' in window);
  }, []);

  // Auto-save effect
  useEffect(() => {
    if (!fileHandle) return;
    
    let isMounted = true;
    
    const autoSave = async () => {
      if (!isMounted) return;
      setSaveStatus('saving');
      try {
        const writable = await fileHandle.createWritable();
        await writable.write(JSON.stringify(resumeData, null, 2));
        await writable.close();
        if (isMounted) {
          setSaveStatus('saved');
          setTimeout(() => {
            if (isMounted) setSaveStatus((prev) => prev === 'saved' ? 'idle' : prev);
          }, 2000);
        }
      } catch (err) {
        console.error('Auto-save failed:', err);
        if (isMounted) setSaveStatus('error');
      }
    };

    const timer = setTimeout(autoSave, 1000);
    return () => {
      isMounted = false;
      clearTimeout(timer);
    };
  }, [resumeData, fileHandle]);

  const handleLegacyExport = () => {
    const dataStr = JSON.stringify(resumeData, null, 2);
    const blob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `cvapp-data-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const handleLegacyImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    try {
      const content = await file.text();
      const json = JSON.parse(content);
      if (json && typeof json === 'object' && 'personalInfo' in json) {
        setResumeData(json);
        alert('CV data loaded successfully!');
      } else {
        alert('Invalid CV data file.');
      }
    } catch (err) {
      console.error('Failed to parse JSON:', err);
      alert('Failed to parse the data file.');
    }
    e.target.value = '';
  };

  const handleOpenFile = async () => {
    try {
      const [handle] = await (window as any).showOpenFilePicker({
        types: [{ description: 'JSON Files', accept: { 'application/json': ['.json'] } }],
      });
      const file = await handle.getFile();
      const content = await file.text();
      const json = JSON.parse(content);
      if (json && typeof json === 'object' && 'personalInfo' in json) {
        setResumeData(json);
        setFileHandle(handle);
      } else {
        alert('Invalid CV data file.');
      }
    } catch (err: any) {
      if (err.name !== 'AbortError') {
        console.error(err);
        alert('Failed to open file.');
      }
    }
  };

  const handleSaveAs = async () => {
    try {
      const handle = await (window as any).showSaveFilePicker({
        suggestedName: `cvapp-data-${new Date().toISOString().split('T')[0]}.json`,
        types: [{ description: 'JSON Files', accept: { 'application/json': ['.json'] } }],
      });
      setFileHandle(handle);
    } catch (err: any) {
      if (err.name !== 'AbortError') {
        console.error(err);
        alert('Failed to link file.');
      }
    }
  };

  if (fileHandle) {
    return (
      <div className="mb-5 p-4 rounded-2xl bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 shadow-sm">
        <div className="flex items-center justify-between mb-3 pb-3 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2 max-w-[70%]">
            <div className={`w-2.5 h-2.5 rounded-full flex-shrink-0 shadow-sm ${saveStatus === 'saving' ? 'bg-amber-500 animate-pulse' : saveStatus === 'error' ? 'bg-red-500' : 'bg-green-500'}`}></div>
            <span className="text-[13px] font-bold tracking-tight text-slate-800 dark:text-slate-200 truncate" title={fileHandle.name}>
              {fileHandle.name}
            </span>
          </div>
          <button onClick={() => setFileHandle(null)} className="text-[10px] py-1.5 px-3 uppercase tracking-widest bg-white dark:bg-slate-700 rounded-full text-slate-600 dark:text-slate-300 hover:bg-red-50 hover:text-red-700 dark:hover:bg-red-500/20 dark:hover:text-red-400 border border-slate-200 dark:border-slate-600 hover:border-red-300 dark:hover:border-red-500/30 transition-all font-bold shadow-sm">
            Unlink
          </button>
        </div>
        <div className="flex items-start gap-3">
          {saveStatus === 'saving' && <i className="bi bi-cloud-arrow-up text-amber-500 text-lg"></i>}
          {saveStatus === 'error' && <i className="bi bi-exclamation-triangle text-red-500 text-lg"></i>}
          {saveStatus === 'saved' && <i className="bi bi-cloud-check text-green-500 text-lg"></i>}
          {saveStatus === 'idle' && <i className="bi bi-cloud-check text-slate-400 text-lg"></i>}
          <p className="text-[11px] font-medium text-slate-500 dark:text-slate-400 leading-snug mt-0.5">
            {saveStatus === 'saving' ? 'Streaming changes to disk...' : saveStatus === 'error' ? 'Error saving file! Check permissions.' : 'Live sync active. Changes stream directly to disk.'}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 gap-3 mb-5">
      <input
        ref={fileRef}
        type="file"
        accept=".json"
        onChange={handleLegacyImport}
        className="hidden"
      />
      <button
        type="button"
        onClick={isSupported ? handleSaveAs : handleLegacyExport}
        className="group flex flex-col items-center justify-center gap-2 p-4 rounded-2xl bg-cyan-50 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-300 hover:bg-cyan-100 hover:shadow-md dark:hover:bg-cyan-900/50 transition-all active:scale-[0.98] border border-transparent hover:border-cyan-200 dark:hover:border-cyan-500/30"
      >
        <svg className="w-6 h-6 text-cyan-600 dark:text-cyan-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
        <span className="text-[11px] font-extrabold uppercase tracking-widest">{isSupported ? 'Save As...' : 'Save Data'}</span>
      </button>

      <button
        type="button"
        onClick={isSupported ? handleOpenFile : () => fileRef.current?.click()}
        className="group flex flex-col items-center justify-center gap-2 p-4 rounded-2xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-100 hover:shadow-md dark:hover:bg-emerald-900/40 transition-all active:scale-[0.98] border border-transparent hover:border-emerald-200 dark:hover:border-emerald-500/30"
      >
        <svg className="w-6 h-6 text-emerald-600 dark:text-emerald-400 group-hover:-translate-y-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
        </svg>
        <span className="text-[11px] font-extrabold uppercase tracking-widest">{isSupported ? 'Open File' : 'Load Data'}</span>
      </button>
    </div>
  );
}
