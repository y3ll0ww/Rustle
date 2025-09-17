use serde::Serialize;

use crate::database::pagination::queries::meta::PaginationMetaData;

#[derive(Serialize)]
pub struct Record<T: Serialize> {
    index: usize,
    data: T,
}

#[derive(Serialize)]
pub struct PaginatedRecords<T: Serialize> {
    records: Vec<Record<T>>,
    total: i64,
    page: i64,
    limit: i64,
    total_pages: i64,
    has_next: bool,
    has_prev: bool,
}

impl<T: Serialize> PaginatedRecords<T> {
    pub fn new(meta: PaginationMetaData, records: Vec<T>) -> Self {
        let low_index = meta.page_limit * (meta.requested_page - 1);
        let upper_bound = records.len() + low_index as usize;
        let lower_bound = ((low_index + 1) as usize).min(upper_bound);

        let records = records
            .into_iter()
            .enumerate()
            .map(|(i, record)| Record {
                index: lower_bound + i,
                data: record,
            })
            .collect();

        PaginatedRecords {
            records,
            total: meta.total_records,
            page: meta.requested_page,
            limit: meta.page_limit,
            total_pages: meta.total_pages,
            has_next: meta.requested_page < meta.total_pages,
            has_prev: meta.requested_page > 1,
        }
    }

    pub fn records_on_page(&self) -> usize {
        self.records.len()
    }

    pub fn total_records(&self) -> i64 {
        self.total
    }
}
