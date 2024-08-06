pub trait Asset {
    // This is treated as an ID
    // Two resources CANNOT have the same file_name
    fn file_name(&self) -> &str;
}
