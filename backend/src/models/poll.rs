use mongodb::bson::{oid::ObjectId, doc};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub question: String,
    pub options: Vec<(ObjectId, String)>,
    pub created_by: String,
    pub created_at: chrono::DateTime<Utc>,
    pub is_multiple_choice: bool,
    pub isactive: bool,
}

impl Poll {
    pub fn _new(
        question: String,
        options: Vec<String>,
        created_by: String,
        is_multiple_choice: bool,
    ) -> Self {
        let options_with_ids = options.into_iter()
            .map(|text| (ObjectId::new(), text))
            .collect();

        Poll {
            id: None,
            question,
            options: options_with_ids,
            created_by,
            created_at: Utc::now(),
            is_multiple_choice,
            isactive: true,
        }
    }
}
