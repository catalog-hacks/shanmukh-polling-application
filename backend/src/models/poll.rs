use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,  // Reference to User
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<i32>,  // Track votes for each option
}

impl Poll {
    pub fn _new(user_id: String, question: String, options: Vec<String>) -> Self {
        Poll {
            id: None,
            user_id,
            question,
            options: options.clone(),
            votes: vec![0; options.len()], // Initialize votes to 0 for each option
        }
    }
}
