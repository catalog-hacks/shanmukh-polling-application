use crate::models::user::User;
use crate::models::{poll::Poll, vote::Vote};
use crate::repositories::{PollRepository, Repository, UserRepository, VoteRepository};
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{Client, Collection};
use std::error::Error;

pub struct MongoDBRepository {
    poll_collection: Collection<Poll>,
    vote_collection: Collection<Vote>,
    user_collection: Collection<User>,
}

impl MongoDBRepository {
    pub fn new(client: &Client) -> Self {
        let db = client.database("polling_app");
        MongoDBRepository {
            poll_collection: db.collection::<Poll>("polls"),
            vote_collection: db.collection::<Vote>("votes"),
            user_collection: db.collection::<User>("users"),

        }
    }
}

#[async_trait]
impl UserRepository for MongoDBRepository {
    async fn store_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        let filter = doc! { "user_id": &user.user_id };
        if self.user_collection.find_one(filter).await?.is_none() {
            self.user_collection.insert_one(user).await?;
        }
        Ok(())
    }

    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>, Box<dyn Error>> {
        let filter = doc! { "user_id": user_id };
        let user = self.user_collection.find_one(filter).await?;
        Ok(user)
    }
}

#[async_trait]
impl PollRepository for MongoDBRepository {
    async fn create_poll(&self, poll: Poll) -> Result<(), Box<dyn Error>> {
        self.poll_collection.insert_one(poll).await?;
        Ok(())
    }

    async fn get_poll_by_id(&self, id: ObjectId) -> Result<Option<Poll>, Box<dyn Error>> {
        let poll = self.poll_collection.find_one(doc! { "_id": id }).await?;
        Ok(poll)
    }

    async fn find_polls_by_ids(
        &self,
        poll_ids: Vec<ObjectId>,
    ) -> Result<Vec<Poll>, Box<dyn Error>> {
        let filter = doc! { "_id": { "$in": poll_ids } };
        let cursor = self.poll_collection.find(filter).await?;
        let polls: Vec<Poll> = cursor.try_collect().await?;
        Ok(polls)
    }

    async fn get_all_polls(&self) -> Result<Vec<Poll>, Box<dyn Error>> {
        let cursor = self.poll_collection.find(doc! {}).await?;
        let polls: Vec<Poll> = cursor.try_collect().await?;
        Ok(polls)
    }

    async fn get_polls_by_user(&self, user_id: &str) -> Result<Vec<Poll>, Box<dyn Error>> {
        let cursor = self.poll_collection.find(doc! { "created_by": user_id }).await?;
        let polls: Vec<Poll> = cursor.try_collect().await?;
        Ok(polls)
    }

    async fn update_poll_status(&self, id: ObjectId, is_active: bool) -> Result<(), Box<dyn Error>> {
        self.poll_collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "isactive": is_active } },
            )
            .await?;
        Ok(())
    }

    async fn reset_votes_for_poll(&self, poll_id: ObjectId) -> Result<(), Box<dyn Error>> {
        self.vote_collection
            .delete_many(doc! { "poll_id": poll_id })
            .await?;
        Ok(())
    }

    async fn get_poll_results(&self, poll_id: ObjectId) -> Result<Vec<(ObjectId, i32)>, Box<dyn Error>> {
        let pipeline = vec![
            doc! { "$match": { "poll_id": poll_id } },
            doc! { "$unwind": "$option_ids" },
            doc! { "$group": { "_id": "$option_ids", "count": { "$sum": 1 } } },
        ];

        let mut cursor = self.vote_collection.aggregate(pipeline).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            let option_id = doc.get_object_id("_id")?;
            let count = doc.get_i32("count").unwrap_or(0);
            results.push((option_id, count));
        }

        Ok(results)
    }
}

#[async_trait]
impl VoteRepository for MongoDBRepository {
    // Find votes by user
    async fn find_votes_by_user(&self, user_id: &str) -> Result<Vec<Vote>, Box<dyn Error>> {
        let cursor = self.vote_collection.find(doc! { "user_id": user_id }).await?;
        let votes: Vec<Vote> = cursor.try_collect().await?;
        Ok(votes)
    }

    // Find a specific vote by poll and user
    async fn find_vote(
        &self,
        poll_id: ObjectId,
        user_id: &str,
    ) -> Result<Option<Vote>, Box<dyn Error>> {
        let filter = doc! { "poll_id": poll_id, "user_id": user_id };
        let vote = self.vote_collection.find_one(filter).await?;
        Ok(vote)
    }

    // Submit or update a vote
    async fn submit_or_update_vote(&self, vote: Vote) -> Result<(), Box<dyn Error>> {
        let filter = doc! {
            "poll_id": vote.poll_id,
            "user_id": vote.user_id.clone()
        };
    
        // Perform update operation
        let existing_vote = self.vote_collection.find_one(filter.clone()).await?;
    
        if let Some(_) = existing_vote {
            // Update existing vote
            let update_result = self.vote_collection
                .update_one(
                    filter,
                    doc! { "$set": { "option_ids": vote.option_ids } },
                )
                .await;
    
            match update_result {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            // Insert new vote
            let insert_result = self.vote_collection.insert_one(vote).await;
    
            match insert_result {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
}

#[async_trait]
impl Repository for MongoDBRepository {}