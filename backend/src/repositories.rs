use crate::models::{poll::Poll, vote::Vote, user::User};
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use std::error::Error;

#[async_trait]
pub trait PollRepository {
    async fn create_poll(&self, poll: Poll) -> Result<(), Box<dyn Error>>;
    async fn get_poll_by_id(&self, id: ObjectId) -> Result<Option<Poll>, Box<dyn Error>>;
    async fn get_all_polls(&self) -> Result<Vec<Poll>, Box<dyn Error>>;
    async fn get_polls_by_user(&self, user_id: &str) -> Result<Vec<Poll>, Box<dyn Error>>;
    async fn update_poll_status(&self, id: ObjectId, is_active: bool) -> Result<(), Box<dyn Error>>;
    async fn reset_votes_for_poll(&self, poll_id: ObjectId) -> Result<(), Box<dyn Error>>;
    async fn get_poll_results(&self, poll_id: ObjectId) -> Result<Vec<(ObjectId, i32)>, Box<dyn Error>>;
    async fn find_polls_by_ids(&self, poll_ids: Vec<ObjectId>) -> Result<Vec<Poll>, Box<dyn Error>>;
}

#[async_trait]
pub trait VoteRepository {
    async fn find_votes_by_user(&self, user_id: &str) -> Result<Vec<Vote>, Box<dyn Error>>;
    async fn find_vote(&self, poll_id: ObjectId, user_id: &str) -> Result<Option<Vote>, Box<dyn Error>>;
    async fn submit_or_update_vote(&self, vote: Vote) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait UserRepository {
    async fn store_user(&self, user: User) -> Result<(), Box<dyn Error>>;
    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>, Box<dyn Error>>;
}

#[async_trait]
pub trait Repository: PollRepository + VoteRepository + UserRepository + Send + Sync {}
