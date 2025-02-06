use serde::{Deserialize, Serialize};
use toml::value::Date;
use crate::kimai::MetaField;
use crate::kimai::teams::Team;

pub mod list;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u32,
    pub name: Option<String>,
    pub parent_title: Option<String>,
    pub comment: Option<String>,
    pub start: Option<Date>,
    pub stop: Option<Date>,
    pub customer: u32,
    pub meta_fields: Vec<MetaField>,
    pub visible: bool,
    pub billable: bool,
    pub teams: Vec<Team>,
}