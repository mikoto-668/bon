use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub user: String,
    pub email: String,
    pub editor: String,
    pub files: HashMap<String, String>,
}
