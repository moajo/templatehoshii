#[derive(Debug, PartialEq, Eq)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub is_single_file: bool,
}
