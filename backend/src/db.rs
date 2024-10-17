use crate::Article;
use sqlx::PgPool;

// Inserts a new article into the articles database
pub async fn create_article(pool: &PgPool, a: Article) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO articles (title, author, content)
        VALUES ($1, $2, $3)
        "#,
        a.title,
        a.author,
        a.content
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Updates an article in the articles database by ID
pub async fn update_article(pool: &PgPool, id: i32, a: Article) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE articles
        SET title = $1, author = $2, content = $3
        WHERE id = $4
        "#,
        a.title,
        a.author,
        a.content,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Deletes an article from the articles database by ID
pub async fn delete_article(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM articles
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Retrieves a specific article from the articles database by ID
pub async fn get_article_by_id(pool: &PgPool, id: i32) -> Result<Article, sqlx::Error> {
    let article = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, author, content
        FROM articles
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(article)
}

// Retrieves all articles from the articles database
pub async fn list_all_articles(pool: &PgPool) -> Result<Vec<Article>, sqlx::Error> {
    let articles = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, author, content
        FROM articles
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}


