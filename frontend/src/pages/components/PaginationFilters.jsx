import "../../style/pagination.css";
import { useEffect, useState } from "react";

export default function PaginationFilters({
  paginationState,
  onChange,
  availableSorts = ["created_at", "updated_at", "name"],
  perPageOptions = [10, 20, 50, 100],
}) {
  const [localFilters, setLocalFilters] = useState(paginationState);

  const [filters, setFilters] = useState({
    search: null,
    limit: 10,
    page: 1,
    sortBy: "created_at",
    sortOrder: "desc",
  });

  const handleChange = (field, value) => {
    const updated = { ...localFilters, [field]: value };
    setLocalFilters(updated);
    setFilters({ ...filters, [field]: value });
    onChange?.(updated); // bubble up changes
  };

  useEffect(() => {
    console.log(filters);
  }, [filters])

  return (
    <div className="pagination-filters">
      {/* Search */}
      <input
        type="text"
        placeholder="Search..."
        value={filters.search || ""}
        onChange={(e) => handleChange("search", e.target.value)}
        className="pf-input"
      />

      {/* Sort by field */}
      <select
        value={filters.sort_by}
        onChange={(e) => handleChange("sort_by", e.target.value)}
        className="pf-select"
      >
        {availableSorts.map((field) => (
          <option key={field} value={field}>
            Sort by {field.replace("_", " ")}
          </option>
        ))}
      </select>

      {/* Sort order */}
      <select
        value={localFilters.sort_order}
        onChange={(e) => handleChange("sort_order", e.target.value)}
        className="pf-select"
      >
        <option value="asc">Ascending ↑</option>
        <option value="desc">Descending ↓</option>
      </select>

      {/* Per-page */}
      <select
        value={localFilters.per_page}
        onChange={(e) => handleChange("per_page", Number(e.target.value))}
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
