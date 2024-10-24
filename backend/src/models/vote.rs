use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub poll_id: ObjectId,
    pub option_ids: Vec<ObjectId>,
    pub user_id: String,
}

impl Vote {
    pub fn _new(poll_id: ObjectId, option_ids: Vec<ObjectId>, user_id: String) -> Self {
        Vote {
            id: None,
            poll_id,
            option_ids,
            user_id,
        }
    }
}
