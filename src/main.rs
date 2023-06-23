mod api;
mod db;
mod middlewares;

use beijing::token_intercept;
use civilization::init_service;
use db::init_db;
use middlewares::MiddlewareLayer;
use tonic::transport::Server;
use api::v1::PostsApiV1;
use posts_proto::posts_v1_server::PostsV1Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (env, addr, _) = init_service();

    let db = init_db(env).await;

    let posts_v1 = PostsV1Server::with_interceptor(PostsApiV1::new(db), token_intercept);
    
    let middleware_layer = tower::ServiceBuilder::new().layer(MiddlewareLayer {}).into_inner();
    
    tracing::event!(tracing::Level::INFO, "Posts app is ready!");

    Server::builder()
        .layer(middleware_layer)
        .add_service(posts_v1)
        .serve(addr)
        .await?;

    Ok(())
}
