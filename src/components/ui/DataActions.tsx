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

    // Debounce auto-save by 1 second to avoid excessive disk writes
    const timer = setTimeout(autoSave, 1000);
    return () => {
      isMounted = false;
      clearTimeout(timer);
    };
  }, [resumeData, fileHandle]);

  // Fallback Export
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

  // Fallback Import
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

  // File System API Open
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

  // File System API Save As
  const handleSaveAs = async () => {
    try {
      const handle = await (window as any).showSaveFilePicker({
        suggestedName: `cvapp-data-${new Date().toISOString().split('T')[0]}.json`,
        types: [{ description: 'JSON Files', accept: { 'application/json': ['.json'] } }],
      });
      setFileHandle(handle);
      // The useEffect will immediately catch the new handle and save the current data
    } catch (err: any) {
      if (err.name !== 'AbortError') {
        console.error(err);
        alert('Failed to link file.');
      }
    }
  };

  if (fileHandle) {
    return (
      <div className="mb-4 p-3.5 rounded-xl bg-slate-100 dark:bg-slate-800/80 border border-emerald-500/30 shadow-sm animate-scale-in">
        <div className="flex items-center justify-between mb-2 pb-2 border-b border-slate-200 dark:border-slate-700/50">
          <div className="flex items-center gap-2 max-w-[75%]">
            <div className={`w-2 h-2 rounded-full flex-shrink-0 ${saveStatus === 'saving' ? 'bg-amber-500 animate-pulse' : saveStatus === 'error' ? 'bg-red-500' : 'bg-emerald-500'}`}></div>
            <span className="text-xs font-semibold text-slate-700 dark:text-slate-300 truncate" title={fileHandle.name}>
              {fileHandle.name}
            </span>
          </div>
          <button onClick={() => setFileHandle(null)} className="text-[10px] py-1 px-2.5 bg-white dark:bg-slate-700 rounded-lg text-slate-600 dark:text-slate-300 hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-500/20 dark:hover:text-red-400 border border-slate-200 dark:border-slate-600 hover:border-red-200 dark:hover:border-red-500/30 transition-all font-semibold shadow-sm">
            Unlink
          </button>
        </div>
        <div className="flex items-start gap-2">
          {saveStatus === 'saving' && <i className="bi bi-cloud-arrow-up text-amber-500 mt-0.5"></i>}
          {saveStatus === 'error' && <i className="bi bi-exclamation-triangle text-red-500 mt-0.5"></i>}
          {saveStatus === 'saved' && <i className="bi bi-cloud-check text-emerald-500 mt-0.5"></i>}
          {saveStatus === 'idle' && <i className="bi bi-cloud-check text-slate-400 mt-0.5"></i>}
          <p className="text-[10.5px] text-slate-500 dark:text-slate-400 leading-snug">
            {saveStatus === 'saving' ? 'Streaming changes to disk...' : saveStatus === 'error' ? 'Error saving file! Check permissions.' : 'Live sync active. Changes are streaming directly to your local file.'}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 gap-2 mb-4">
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
        className="group flex flex-col items-center justify-center gap-1.5 p-3 rounded-xl bg-slate-100 dark:bg-slate-800/80 hover:bg-white dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 transition-all hover:shadow-md border border-slate-200 dark:border-slate-700/50 hover:border-indigo-300 dark:hover:border-indigo-500/30"
      >
        <svg className="w-5 h-5 text-indigo-500 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
        <span className="text-[10px] font-bold uppercase tracking-wider">{isSupported ? 'Save As...' : 'Save Data'}</span>
      </button>

      <button
        type="button"
        onClick={isSupported ? handleOpenFile : () => fileRef.current?.click()}
        className="group flex flex-col items-center justify-center gap-1.5 p-3 rounded-xl bg-slate-100 dark:bg-slate-800/80 hover:bg-white dark:hover:bg-slate-800 text-slate-700 dark:text-slate-300 transition-all hover:shadow-md border border-slate-200 dark:border-slate-700/50 hover:border-emerald-300 dark:hover:border-emerald-500/30"
      >
        <svg className="w-5 h-5 text-emerald-500 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
        </svg>
        <span className="text-[10px] font-bold uppercase tracking-wider">{isSupported ? 'Open File' : 'Load Data'}</span>
      </button>
    </div>
  );
}
