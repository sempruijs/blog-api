use sqlx::PgPool;
use warp::Filter;

use crate::handlers::*;

pub async fn serve_routes(pool: PgPool) {
    // Clone the pool to share it across routes
    let pool_filter = warp::any().map(move || pool.clone());

    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin
        .allow_header("content-type") // Allow specific headers
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]); // Allow specific methods

    let create_article = warp::post()
        .and(warp::path("articles"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(create_article_handler);

    let update_article = warp::put()
        .and(warp::path("articles"))
        .and(warp::path::param()) // Extracts the article ID from the URL path
        .and(warp::body::json()) // The updated article data in JSON format
        .and(pool_filter.clone())
        .and_then(update_article_handler);

    let delete_article = warp::delete()
        .and(warp::path("articles"))
        .and(warp::path::param()) // Extracts the article ID from the URL path
        .and(pool_filter.clone())
        .and_then(delete_article_handler);

    let get_article_by_id = warp::get()
        .and(warp::path("articles"))
        .and(warp::path::param()) // Extracts the article ID from the URL path
        .and(pool_filter.clone())
        .and_then(get_article_by_id_handler);

    let create_user = warp::post()
        .and(warp::path("users"))  // Matches the "users" path
        .and(warp::body::json())   // Extracts the JSON body as a `User`
        .and(pool_filter.clone())  // Passes the database connection pool
        .and_then(create_user_handler);


    // Combine all the routes
    let routes = create_article
        .or(update_article)
        .or(delete_article)
        .or(get_article_by_id)
        .or(create_user)
        .with(cors);

    println!("Starting server");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
