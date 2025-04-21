use serde::Serialize;

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
    pub fn new(total: i64, page: i64, limit: i64, total_pages: i64) -> Self {
        PaginatedRecords {
            records: Vec::new(),
            total,
            page,
            limit,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }

    pub fn add_records(mut self, data: Vec<T>) -> Self {
        let low_index = self.limit * (self.page - 1);
        let upper_bound = data.len() + low_index as usize;
        let lower_bound = ((low_index + 1) as usize).min(upper_bound);

        self.records = data
            .into_iter()
            .enumerate()
            .map(|(i, record)| Record {
                index: lower_bound + i,
                data: record,
            })
            .collect();

        self
    }

    pub fn records_on_page(&self) -> usize {
        self.records.len()
    }

    pub fn total_records(&self) -> i64 {
        self.total
    }
}
