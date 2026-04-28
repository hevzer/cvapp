export default function ExportButton() {
  const handleExport = () => {
    window.print();
  };

  return (
    <button
      type="button"
      onClick={handleExport}
      className="w-full flex items-center justify-center gap-3 px-6 py-4.5 rounded-full text-[13px] font-bold tracking-widest uppercase text-white bg-red-600 hover:bg-red-700 shadow-md shadow-red-500/30 hover:shadow-lg hover:shadow-red-500/50 hover:-translate-y-0.5 active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed transition-all duration-300"
    >
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      Export to PDF
    </button>
  );
}
