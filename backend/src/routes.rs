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

    let list_all_articles = warp::get()
        .and(warp::path("articles"))
        .and(warp::path::end()) // Ensure this route doesn't have an ID path parameter
        .and(pool_filter.clone())
        .and_then(list_all_articles_handler);


    // Combine all the routes
    // let routes = create_user.or(list_users).or(delete_user).or(update_user).with(cors);
    let routes = create_article
        .or(update_article)
        .or(delete_article)
        .or(get_article_by_id)
        .or(list_all_articles)
        .with(cors);

    println!("Starting server");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
