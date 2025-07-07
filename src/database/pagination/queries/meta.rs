use crate::database::pagination::{request::PaginationRequest, sort::SortField};

pub struct PaginationMetaData {
    pub total_records: i64,
    pub total_pages: i64,
    pub requested_page: i64,
    pub record_offset: i64,
    pub page_limit: i64,
}

impl PaginationMetaData {
    pub fn new<T: SortField>(total_records: i64, params: &PaginationRequest<T>) -> Self {
        // Number of maximum results (default 20, min 1, max 100)
        let page_limit = params.limit.unwrap_or(20).clamp(1, 100);

        // Define the total number of pages by dividing the total by the limit and returning the upper
        // bound from the float as integer. Make sure there is at least one page.
        let total_pages = ((total_records as f64 / page_limit as f64).ceil() as i64).max(1);

        // Number of the page (should be at least 1) and capped to total pages
        let requested_page = params.page.unwrap_or(1).max(1).min(total_pages);

        // Calculate the offset of the search
        let record_offset = (requested_page - 1) * page_limit;

        PaginationMetaData {
            total_records,
            total_pages,
            requested_page,
            record_offset,
            page_limit,
        }
    }
}
