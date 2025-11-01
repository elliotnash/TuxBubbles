pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn name(&self) -> &str {
        match self {
            SortOrder::Ascending => "ASC",
            SortOrder::Descending => "DESC",
        }
    }
}
