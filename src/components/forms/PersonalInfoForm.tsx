import { useRef } from 'react';
import { useResumeStore } from '@/store/useResumeStore';
import MaterialInput from '@/components/ui/MaterialInput';
import MaterialTextarea from '@/components/ui/MaterialTextarea';

export default function PersonalInfoForm() {
  const personalInfo = useResumeStore((s) => s.resumeData.personalInfo);
  const updatePersonalInfo = useResumeStore((s) => s.updatePersonalInfo);
  const photoInputRef = useRef<HTMLInputElement>(null);

  const handlePhotoUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    if (!file.type.startsWith('image/')) {
      alert('Please select an image file.');
      return;
    }

    if (file.size > 5 * 1024 * 1024) {
      alert('Image must be under 5MB.');
      return;
    }

    const reader = new FileReader();
    reader.onload = (ev) => {
      const dataUrl = ev.target?.result as string;
      updatePersonalInfo({ photoUrl: dataUrl });
    };
    reader.readAsDataURL(file);
    e.target.value = '';
  };

  const fields = [
    { key: 'fullName', label: 'Full Name', type: 'text', placeholder: 'John Doe' },
    { key: 'title', label: 'Professional Title', type: 'text', placeholder: 'Senior Software Engineer' },
    { key: 'email', label: 'Email', type: 'email', placeholder: 'john@example.com' },
    { key: 'phone', label: 'Phone', type: 'tel', placeholder: '+1 (555) 123-4567' },
    { key: 'location', label: 'Location', type: 'text', placeholder: 'New York, NY' },
    { key: 'linkedin', label: 'LinkedIn', type: 'text', placeholder: 'linkedin.com/in/johndoe' },
    { key: 'github', label: 'GitHub', type: 'text', placeholder: 'github.com/johndoe' },
    { key: 'website', label: 'Website', type: 'text', placeholder: 'johndoe.dev' },
    { key: 'drivingLicense', label: 'Driving License', type: 'text', placeholder: 'Class C' },
  ] as const;

  return (
    <div className="space-y-3">
      {/* Photo Upload */}
      <div>
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Profile Photo
        </label>
        <div className="flex items-center gap-4">
          <div
            onClick={() => photoInputRef.current?.click()}
            className="relative w-20 h-20 rounded-full overflow-hidden border-2 border-dashed border-gray-300 dark:border-gray-600 cursor-pointer group hover:border-cyan-400 dark:hover:border-cyan-500 transition-all duration-300 flex-shrink-0"
          >
            {personalInfo.photoUrl ? (
              <>
                <img
                  src={personalInfo.photoUrl}
                  alt="Profile"
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity duration-200 flex items-center justify-center">
                  <span className="text-white text-xs font-medium">Change</span>
                </div>
              </>
            ) : (
              <div className="w-full h-full bg-gray-100 dark:bg-gray-700 flex flex-col items-center justify-center gap-0.5 group-hover:bg-cyan-50 dark:group-hover:bg-cyan-950/30 transition-colors duration-200">
                <svg className="w-5 h-5 text-gray-400 group-hover:text-cyan-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
                <span className="text-[10px] text-gray-400 group-hover:text-cyan-500 transition-colors">Photo</span>
              </div>
            )}
          </div>
          <div className="flex flex-col gap-1.5">
            <button
              type="button"
              onClick={() => photoInputRef.current?.click()}
              className="text-xs font-medium text-cyan-600 dark:text-cyan-400 hover:text-cyan-700 dark:hover:text-cyan-300 active:scale-95 transition-all duration-150"
            >
              {personalInfo.photoUrl ? 'Change photo' : 'Upload photo'}
            </button>
            {personalInfo.photoUrl && (
              <button
                type="button"
                onClick={() => updatePersonalInfo({ photoUrl: '' })}
                className="text-xs text-red-500 hover:text-red-600 dark:text-red-400 dark:hover:text-red-300 font-medium active:scale-95 transition-all duration-150"
              >
                Remove
              </button>
            )}
            <span className="text-[10px] text-gray-400">JPG, PNG. Max 5MB.</span>
          </div>
        </div>
        <input
          ref={photoInputRef}
          type="file"
          accept="image/*"
          onChange={handlePhotoUpload}
          className="hidden"
        />
      </div>

      {/* Objective */}
      <div className="pt-2">
        <MaterialTextarea
          label="Objective / Looking For"
          value={personalInfo.objective || ''}
          onChange={(e) => updatePersonalInfo({ objective: e.target.value })}
          rows={3}
        />
      </div>

      {/* Text Fields */}
      <div className="space-y-4 pt-2">
        {fields.map((field) => (
          <MaterialInput
            key={field.key}
            type={field.type}
            label={field.label}
            value={(personalInfo[field.key as keyof typeof personalInfo] as string) || ''}
            onChange={(e) => updatePersonalInfo({ [field.key]: e.target.value })}
          />
        ))}
      </div>
    </div>
  );
}
