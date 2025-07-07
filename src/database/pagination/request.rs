use serde::{Deserialize, Serialize};

use super::sort::{SortDirection, SortField};

#[derive(Clone, Deserialize, Serialize)]
pub struct PaginationRequest<F: SortField> {
    pub page: Option<i64>,               // default = 1
    pub limit: Option<i64>,              // default = 20
    pub search: Option<String>,          // optional text search (e.g. name)
    pub sort_by: Option<F>,              // e.g. "created_at", "name"
    pub sort_dir: Option<SortDirection>, // "asc" or "desc"
}
