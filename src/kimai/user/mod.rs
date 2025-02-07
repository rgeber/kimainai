use serde::{Deserialize, Serialize};
use crate::kimai::Preference;

pub mod me;

/// Scaled down user object.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    username: Option<String>,
    account_number: Option<String>,
    enabled: bool,
    api_token: bool,
    preferences: Vec<Preference>,
    language: Option<String>,
    timezone: Option<String>,
    initials: Option<String>,
    alias: Option<String>,
    title: Option<String>,
}