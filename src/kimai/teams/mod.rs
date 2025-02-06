use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    id: u32,
    name: String,
    color: Option<String>,
}