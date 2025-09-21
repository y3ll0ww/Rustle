import { useState } from "react";
import { defaultPagination } from "../utils/ApiHandler";

export function usePagination(initial = {}) {
    const [paginationState, setPaginationState] = useState({
        hasNext: false,
        hasPrev: false,
        limit: initial.per_page ?? defaultPagination.per_page,
        page: initial.page ?? defaultPagination.page,
        records: [],
        sortBy: initial.sort_by ?? defaultPagination.sort_by,
        sortOrder: initial.sort_order ?? defaultPagination.sort_order,
        total: 0,
        totalPages: 0,
    });

    // Update from API response
    const updateFromResponse = (data) => {
        if (!data) {
            setPaginationState((prev) => ({ ...prev }));
            return;
        }
        console.log(data.records);
        setPaginationState((prev) => ({
            ...prev,
            hasNext: data.has_next,
            hasPrev: data.has_prev,
            limit: data.limit,
            page: data.page,
            records: data.records,
            total: data.total,
            totalPages: data.total_pages,
        }));
        console.log(paginationState);
    };

    // Helpers
    const nextPage = () => setPaginationState((prev) => ({ ...prev, page: prev.page + 1 }));
    const prevPage = () => setPaginationState((prev) => ({ ...prev, page: Math.max(1, prev.page - 1) }));
    const reset = () => setPaginationState({ ...defaultPagination, hasNext: false, hasPrev: false, total: 0, totalPages: 0 });

    return { paginationState, setPaginationState, updateFromResponse, nextPage, prevPage, reset };
}
