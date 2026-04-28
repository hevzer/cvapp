import React from 'react';

interface MaterialTextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label: string;
}

export default function MaterialTextarea({ label, className, id, rows = 3, ...props }: MaterialTextareaProps) {
  const generatedId = React.useId();
  const textareaId = id || generatedId;
  
  return (
    <div className={`relative ${className || ''}`}>
      <textarea
        id={textareaId}
        rows={rows}
        {...props}
        className={`block px-3 pb-2.5 pt-4 w-full text-sm text-gray-900 bg-transparent rounded-lg border-2 border-slate-300 appearance-none dark:text-white dark:border-slate-600 dark:focus:border-cyan-500 focus:outline-none focus:ring-0 focus:border-cyan-600 peer transition-colors resize-y min-h-[80px] ${className || ''}`}
        placeholder=" "
      />
      <label
        htmlFor={textareaId}
        className="absolute text-sm font-medium text-slate-500 dark:text-slate-400 duration-300 transform -translate-y-4 scale-75 top-2 z-10 origin-[0] bg-white dark:bg-slate-800 px-2 peer-focus:px-2 peer-focus:text-cyan-600 peer-focus:dark:text-cyan-400 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-[1.2rem] peer-focus:top-2 peer-focus:scale-75 peer-focus:-translate-y-4 start-1 pointer-events-none"
      >
        {label}
      </label>
    </div>
  );
}
