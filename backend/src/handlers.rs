use crate::db::*;
use crate::User;
use sqlx::PgPool;
use warp::http::StatusCode;

use chrono::Utc;

fn current_time_iso8601() -> String {
    Utc::now().to_rfc3339()
}

pub async fn create_user_handler(
    user: User,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601();
    println!("New user created: {:?}  ({})", user, now);

    match create_user(&pool, user).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => panic!("error while listing users"),
    }
}

pub async fn list_users_handler(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    
    let now = current_time_iso8601();
    println!("Listing users. ({})", now);
    match read_users(&pool).await {
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(_) => panic!("error while listing users"),
    }
}

pub async fn update_user_handler(
    id: i32,
    updated_user: User,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601();
    println!("User with id: {} will be updated to: {:?} {}", id, updated_user, now);
    
    match update_user(&pool, id, &updated_user).await {
        Ok(_) => {
            println!("User updated");
            Ok(StatusCode::OK)
        },
        Err(_) => panic!("Error updating user"),
    }
}

pub async fn delete_user_handler(
    id: i32,
    pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let now = current_time_iso8601();
    println!("User with id: {} deleted. ({})", id, now);
    
    match delete_user(&pool, id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => panic!("error while deleting user"),
    }
}
