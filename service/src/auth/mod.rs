pub mod access;
pub mod domain;
pub mod role;
pub mod user;
pub mod user_role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchNameStruct {
    pub name: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostUserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
