import "../../style/pagination.css";
import { ChevronLeft, ChevronRight } from "lucide-react";

export default function PaginationControls({ paginationState, onPrev, onNext }) {
  if (!paginationState) return null;

  return (
    <div className="pagination-controls">
      <button
        onClick={onPrev}
        disabled={!paginationState.hasPrev}
        className="pc-btn"
        aria-label="Previous page"
      >
        <ChevronLeft className="icon-left" />
        Prev
      </button>

      <span className="pc-info">
        Page {paginationState.page} of {paginationState.totalPages} · <strong>{paginationState.total}</strong> total
      </span>

      <button
        onClick={onNext}
        disabled={!paginationState.hasNext}
        className="pc-btn"
        aria-label="Next page"
      >
        Next
        <ChevronRight className="icon-right" />
      </button>
    </div>
  );
}
