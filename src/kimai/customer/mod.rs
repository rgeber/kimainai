use serde::{Deserialize, Serialize};
use crate::kimai::MetaField;
use crate::kimai::teams::Team;

pub mod list;



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: u32,
    pub name: Option<String>,
    pub number: Option<String>,
    pub comment: Option<String>,
    pub visible: bool,
    pub billable: bool,
    pub currency: Option<String>,
    pub meta_fields: Vec<MetaField>,
    pub teams: Vec<Team>,
}
