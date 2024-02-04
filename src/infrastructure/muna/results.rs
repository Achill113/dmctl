use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetNamesResult {
    pub count: i16,
    pub names: Vec<String>,
}
