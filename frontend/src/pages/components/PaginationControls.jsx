import { ChevronLeft, ChevronRight } from "lucide-react";

export default function PaginationControls({ pagination, onPrev, onNext }) {
  if (!pagination) return null;

  return (
    <div className="flex items-center justify-between gap-4 mt-6">
      {/* Prev button */}
      <button
        onClick={onPrev}
        disabled={!pagination.hasPrev}
        className={`flex items-center gap-1 px-3 py-1.5 rounded-lg border text-sm 
          ${pagination.hasPrev 
            ? "hover:bg-gray-100 border-gray-300 text-gray-700" 
            : "border-gray-200 text-gray-400 cursor-not-allowed"}`}
      >
        <ChevronLeft className="h-4 w-4" />
        Prev
      </button>

      {/* Info */}
      <span className="text-sm text-gray-600">
        Page {pagination.page} of {pagination.totalPages} ·{" "}
        <strong>{pagination.total}</strong> total
      </span>

      {/* Next button */}
      <button
        onClick={onNext}
        disabled={!pagination.hasNext}
        className={`flex items-center gap-1 px-3 py-1.5 rounded-lg border text-sm 
          ${pagination.hasNext 
            ? "hover:bg-gray-100 border-gray-300 text-gray-700" 
            : "border-gray-200 text-gray-400 cursor-not-allowed"}`}
      >
        Next
        <ChevronRight className="h-4 w-4" />
      </button>
    </div>
  );
}
