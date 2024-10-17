use crate::db::*;
use crate::Article;
use sqlx::PgPool;
// use warp::http::StatusCode;

use chrono::Utc;

fn current_time_iso8601() -> String {
    Utc::now().to_rfc3339()
}


pub async fn create_article_handler(
    article: Article,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601();
    println!("New article created: {:?}  ({})", article, now);

    match create_article(&pool, article).await {
        Ok(_) => Ok(warp::reply::with_status("Article created", warp::http::StatusCode::CREATED)),
        Err(_) => panic!("failed to add user"),
    }
}

pub async fn update_article_handler(
    id: i32,
    article: Article,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match update_article(&pool, id, article).await {
        Ok(_) => Ok(warp::reply::with_status("Article updated", warp::http::StatusCode::OK)),
        Err(_) => panic!("failed in updating the article"),
    }
}

pub async fn delete_article_handler(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match delete_article(&pool, id).await {
        Ok(_) => Ok(warp::reply::with_status("Article deleted", warp::http::StatusCode::OK)),
        Err(_) => panic!("failed to delete article"),
    }
}

pub async fn get_article_by_id_handler(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match get_article_by_id(&pool, id).await {
        Ok(article) => Ok(warp::reply::json(&article)),
        Err(_) => panic!("failed to recieve specific article"),
    }
}

pub async fn list_all_articles_handler(
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match list_all_articles(&pool).await {
        Ok(articles) => Ok(warp::reply::json(&articles)),
        Err(_) => panic!("failed to list all users"),
    }
}


