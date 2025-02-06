use serde::{Deserialize, Serialize};

pub mod customer;
pub mod api;
pub mod teams;
pub mod projects;
pub mod activities;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  MetaField {}