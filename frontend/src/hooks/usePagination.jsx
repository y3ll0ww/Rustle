import { useState } from "react";
import { defaultPagination } from "../utils/ApiHandler";

export const FIELD_LIMIT = "limit";
export const FIELD_SORT_BY = "sortBy";
export const FIELD_SEARCH = "search";
export const FIELD_SORT_ORDER = "sortOrder";

export function usePagination(initial = {}) {
    const [paginationState, setPaginationState] = useState({
        hasNext: false,
        hasPrev: false,
        search: null,
        limit: initial.per_page ?? defaultPagination.per_page,
        page: initial.page ?? defaultPagination.page,
        records: [],
        sortBy: initial.sort_by ?? defaultPagination.sort_by,
        sortOrder: initial.sort_order ?? defaultPagination.sort_order,
        total: 0,
        totalPages: 0,
    });

    const filters = () => ({
        search: paginationState.search,
        limit: paginationState.limit,
        page: paginationState.page,
        sortBy: paginationState.sortBy,
        sortOrder: paginationState.sortOrder,
    });

    const setFilters = (field, value) => {
        const newValue = field === "search" && value == "" ? null : value;
        setPaginationState((prev) => ({ ...prev, [field]: newValue}));
    };

    // Update from API response
    const updateFromResponse = (data) => {
        if (!data) {
            setPaginationState((prev) => ({ ...prev }));
            return;
        }

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
    };

    // Helpers
    const nextPage = () => setPaginationState((prev) => ({ ...prev, page: prev.page + 1 }));
    const prevPage = () => setPaginationState((prev) => ({ ...prev, page: Math.max(1, prev.page - 1) }));
    const clearSearch = () => setPaginationState((prev) => ({ ...prev, search: null }));
    const reset = () => setPaginationState({
        ...defaultPagination,
        hasNext: false,
        hasPrev: false,
        total: 0,
        totalPages: 0
    });

    return {
        paginationState,
        setPaginationState,
        filters,
        setFilters,
        updateFromResponse,
        nextPage,
        prevPage,
        clearSearch,
        reset
    };
}
