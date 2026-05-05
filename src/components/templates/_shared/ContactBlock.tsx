import 'bootstrap-icons/font/bootstrap-icons.css';
import type { PersonalInfo } from '@/types/resume';
import { i18n, getSafeLanguage } from '@/lib/i18n';

interface ContactBlockProps {
  personalInfo: Partial<PersonalInfo>;
  lang: string;
  /** Tailwind class for the surrounding ul (e.g. layout, gap, color theming). */
  listClassName?: string;
  /** Tailwind class for each li row. */
  rowClassName?: string;
  /** Tailwind class for the icon span (color, sizing, margin). */
  iconClassName?: string;
  /** Tailwind class for the visible value text. */
  valueClassName?: string;
}

/**
 * Renders contact details as a semantic <ul> with:
 *   - aria-hidden icons (visually decorative, ignored by ATS/screen readers)
 *   - sr-only text labels (hidden visually, surfaced to ATS — "Email: ...")
 *   - real text values
 *
 * ATS parsers extract clean strings ("Email: foo@bar.com") instead of
 * Unicode garbage from icon-font glyphs interleaved with values.
 */
export default function ContactBlock({
  personalInfo,
  lang,
  listClassName = '',
  rowClassName = '',
  iconClassName = '',
  valueClassName = '',
}: ContactBlockProps) {
  const safeLang = getSafeLanguage(lang);
  const labelDriving = i18n[safeLang].drivingLicense;

  type Row = { label: string; icon: string; value?: string; breakAll?: boolean };
  const rows: Row[] = [
    { label: 'Email', icon: 'bi-envelope', value: personalInfo.email, breakAll: true },
    { label: 'Phone', icon: 'bi-telephone', value: personalInfo.phone },
    { label: 'Location', icon: 'bi-geo-alt', value: personalInfo.location },
    { label: 'LinkedIn', icon: 'bi-linkedin', value: personalInfo.linkedin, breakAll: true },
    { label: 'GitHub', icon: 'bi-github', value: personalInfo.github, breakAll: true },
    { label: 'Website', icon: 'bi-globe', value: personalInfo.website, breakAll: true },
    {
      label: labelDriving,
      icon: 'bi-car-front',
      value: personalInfo.drivingLicense,
    },
  ];

  const visible = rows.filter((r) => r.value);
  if (visible.length === 0) return null;

  return (
    <ul className={listClassName}>
      {visible.map((r) => (
        <li key={r.label} className={rowClassName}>
          <span aria-hidden="true" className={iconClassName}>
            <i className={`bi ${r.icon}`}></i>
          </span>
          <span className="sr-only">{r.label}: </span>
          <span className={`${valueClassName}${r.breakAll ? ' break-all' : ''}`}>{r.value}</span>
        </li>
      ))}
    </ul>
  );
}
