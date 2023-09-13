use std::collections::HashMap;

pub struct Config {
    pub user: String,
    pub email: String,
    pub editor: String,
    pub files: HashMap<String, String>
}
