use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExampleUser {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>
}