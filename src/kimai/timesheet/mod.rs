use serde::{Deserialize, Serialize};

/// Timesheet's are the actual "work reports"
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeSheet {
    pub begin: String,
    pub end: String,
    pub project: u32,
    pub activity: u32,
    pub description: Option<String>,
    #[serde(rename = "fixedRate")]
    pub fixed_rate: u32,
    #[serde(rename = "hourlyRate")]
    pub hourly_rate: u32,
    pub user: u32,
    pub tags: Option<String>,
    pub exported: bool,
    pub billable: bool,
}