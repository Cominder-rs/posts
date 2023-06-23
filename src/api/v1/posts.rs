use beijing::UserIdScopes;
use civilization::utils::make_internal;
use posts_proto::{posts_v1_server::PostsV1, NewPost, Post, Posts, PostId};
use sea_orm::{prelude::*, ActiveValue::{Set, NotSet}};
use tonic::{Request, Response, Status};
use posts_entities::posts::{ActiveModel as NewPostEntity};

pub struct PostsApiV1 {
    db: DatabaseConnection,
} 

impl PostsApiV1 {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl PostsV1 for PostsApiV1 {
    async fn create_post(&self, req: Request<NewPost>) -> Result<Response<PostId>, Status> {
        let new_post = req.get_ref().to_owned();
        let UserIdScopes { user_id, .. } = req.extensions().get::<UserIdScopes>().ok_or(make_internal("Getting UserIdScopes extensions"))?;

        let new_post = NewPostEntity {
            title: Set(new_post.title),
            short_description: Set(new_post.short_description),
            detailed_description: Set(new_post.detailed_description),
            contacts: Set(posts_entities::posts::StringVec(new_post.contacts)),
            user_id: Set(*user_id),
            category: Set(new_post.category as i16),
            ..Default::default()
        };

        let last_id = sea_orm::Insert::one(new_post).exec_without_returning(&self.db).await.map_err(make_internal)?;

        Ok(
            Response::new(
                PostId {
                    id: last_id as i64
                }
            )
        )

    }

    async fn get_posts(&self, req: Request<()>) -> Result<Response<Posts>, Status> {
        Err(Status::unimplemented(""))
    }
}