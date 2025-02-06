use serde::{Deserialize, Serialize};

pub mod customer;
pub mod api;
pub mod teams;
pub mod projects;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  MetaField {}