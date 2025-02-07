use serde::{Deserialize, Serialize};

pub mod customer;
pub mod api;
pub mod teams;
pub mod projects;
pub mod activities;
pub mod workday;
pub mod user;
pub mod timesheet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  MetaField {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  Preference {
    name: Option<String>,
    value: Option<String>
}
