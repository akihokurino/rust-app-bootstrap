pub struct Pager {
    pub page: i64,
    pub limit: i64,
}
impl Pager {
    pub fn new(page: Option<i64>, limit: Option<i64>) -> Self {
        Self {
            page: page.unwrap_or(1),
            limit: limit.unwrap_or(20),
        }
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }
}
