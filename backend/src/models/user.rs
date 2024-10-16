use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
}

impl User {
    pub fn _new(user_id: String, name: String) -> Self {
        User {
            id: None,
            user_id,
            name,
        }
    }
}
