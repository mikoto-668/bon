use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub user: String,
    pub email: String,
    pub editor: String,
    pub files: BTreeMap<String, String>,
}
