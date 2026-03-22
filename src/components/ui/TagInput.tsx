'use client';

import { useState, type KeyboardEvent } from 'react';

interface TagInputProps {
  tags: string[];
  onChange: (tags: string[]) => void;
  placeholder?: string;
  label?: string;
}

export default function TagInput({
  tags = [],
  onChange,
  placeholder = 'Type and press Enter...',
  label,
}: TagInputProps) {
  const [input, setInput] = useState('');
  const [isFocused, setIsFocused] = useState(false);

  const addTag = (value: string) => {
    const trimmed = value.trim();
    if (trimmed && !tags.includes(trimmed)) {
      onChange([...tags, trimmed]);
    }
    setInput('');
  };

  const removeTag = (index: number) => {
    onChange(tags.filter((_, i) => i !== index));
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      addTag(input);
    } else if (e.key === 'Backspace' && input === '' && tags.length > 0) {
      removeTag(tags.length - 1);
    }
  };

  const active = isFocused || tags.length > 0 || input.length > 0;

  return (
    <div className="relative pt-2">
      <div 
        className={`flex flex-wrap gap-2 px-3 pb-2.5 pt-4 border-2 rounded-lg bg-transparent min-h-[58px] transition-colors ${
          isFocused 
            ? 'border-cyan-600 dark:border-cyan-500' 
            : 'border-slate-300 dark:border-slate-600'
        }`}
      >
        {tags.map((tag, index) => (
          <span
            key={index}
            className="flex items-center gap-1 px-2.5 py-1 bg-cyan-100 dark:bg-cyan-900/50 text-cyan-700 dark:text-cyan-300 text-[13px] font-bold tracking-wide rounded-md"
          >
            {tag}
            <button
              type="button"
              onClick={() => removeTag(index)}
              className="ml-0.5 text-cyan-400 hover:text-cyan-600 dark:hover:text-cyan-200 transition-colors"
            >
              ×
            </button>
          </span>
        ))}
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          placeholder={tags.length === 0 && isFocused ? placeholder : ''}
          className="flex-1 min-w-[120px] outline-none bg-transparent text-sm text-slate-900 dark:text-white placeholder-slate-400"
        />
      </div>
      
      {label && (
        <label
          className={`absolute text-sm font-medium duration-300 transform -translate-y-4 scale-75 top-[1rem] z-10 origin-[0] bg-white dark:bg-slate-800 px-2 start-1 pointer-events-none transition-all ${
            active 
              ? 'text-cyan-600 dark:text-cyan-400 top-2 scale-75 -translate-y-4' 
              : 'text-slate-500 dark:text-slate-400 top-1/2 scale-100 -translate-y-1/2'
          }`}
        >
          {label}
        </label>
      )}
    </div>
  );
}
