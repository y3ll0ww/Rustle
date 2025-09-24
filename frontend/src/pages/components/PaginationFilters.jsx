import "../../style/pagination.css";
import { useEffect, useState } from "react";

const defaultSort = [
  { "date created": "created_at" },
  { "date updated": "updated_at" },
  { "email": "email" },
  { "last name": "last_name" },
  { "first name": "first_name" },
  { "username": "username" },
  { "role": "role" },
  { "status": "status" },
]

export default function PaginationFilters({
  paginationState,
  setFilters,
  availableSorts = defaultSort,
  perPageOptions = [10, 20, 50, 100],
}) {
  const [search, setSearch] = useState("");

  // Delay for search field
  useEffect(() => {
    const handler = setTimeout(() => {
      setFilters("search", search);
    }, 1000);

    return () => clearTimeout(handler);
  }, [search]);

  return (
    <div className="pagination-filters">
      {/* Search */}
      <input
        type="text"
        placeholder="Search..."
        value={search || ""}
        onChange={(e) => setSearch(e.target.value)}
        className="pf-input"
      />

      {/* Sort by field */}
      <select
        value={paginationState.sortBy}
        onChange={(e) => setFilters("sortBy", e.target.value)}
        className="pf-select"
      >
        {availableSorts.map((fieldObj) => {
          const label = Object.keys(fieldObj)[0];
          const value = fieldObj[label];
          return (
            <option key={value} value={value}>
              Sort by {label}
            </option>
          );
        })}
      </select>

      {/* Sort order */}
      <select
        value={paginationState.sortOrder}
        onChange={(e) => setFilters("sort_order", e.target.value)}
        className="pf-select"
      >
        <option value="asc">Ascending ↑</option>
        <option value="desc">Descending ↓</option>
      </select>

      {/* Per-page */}
      <select
        value={paginationState.limit}
        onChange={(e) => setFilters("limit", Number(e.target.value))}
        className="pf-select"
      >
        {perPageOptions.map((n) => (
          <option key={n} value={n}>
            {n} per page
          </option>
        ))}
      </select>
    </div>
  );
}
