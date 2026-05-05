// Localized "Mar 2024" / "mars 2024" — used by templates that prefer human readability.
export function formatDate(date: string, lang: string): string {
  if (!date) return '';
  const [year, month] = date.split('-');
  if (!month) return year;
  try {
    const d = new Date(parseInt(year), parseInt(month) - 1);
    return new Intl.DateTimeFormat(lang, { month: 'short', year: 'numeric' }).format(d);
  } catch {
    return `${month}/${year}`;
  }
}

// Numeric "03/2024" — preferred by ATS parsers (universal regex `\d{2}/\d{4}`).
export function formatDateNumeric(date: string): string {
  if (!date) return '';
  const [year, month] = date.split('-');
  if (!month) return year;
  return `${month.padStart(2, '0')}/${year}`;
}
