use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,  // Reference to User
    pub poll_id: String,  // Reference to Poll
    pub option_index: usize, // Index of the voted option
}

impl Vote {
    pub fn _new(user_id: String, poll_id: String, option_index: usize) -> Self {
        Vote {
            id: None,
            user_id,
            poll_id,
            option_index,
        }
    }
}
