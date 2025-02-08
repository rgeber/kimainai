use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::kimai::MetaField;

/// Reduced timesheet as a full one would be rejeced by the API
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSheetPostable {
    pub begin: DateTime<Local>,
    pub end: DateTime<Local>,
    pub project: u32,
    pub activity: u32,
    pub description: String,
    pub tags: String,
}

/// Full size timesheet (can't post)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSheetResponse {
    pub id: u32,
    pub duration: u32,
    pub begin: DateTime<Local>,
    pub end: DateTime<Local>,
    pub project: u32,
    pub activity: u32,
    pub description: Option<String>,
    pub rate: f64,
    #[serde(rename = "internalRate")]
    pub internal_rate: f64,
    #[serde(rename = "fixedRate")]
    pub fixed_rate: Option<f64>,
    #[serde(rename = "hourlyRate")]
    pub hourly_rate: Option<f64>,
    pub user: u32,
    pub tags: Vec<String>,
    pub exported: bool,
    pub billable: bool,
    #[serde(rename = "metaFields")]
    pub meta_fields: Vec<MetaField>
}