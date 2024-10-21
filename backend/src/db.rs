use crate::{Article, User};
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, u: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, password)
        VALUES ($1, $2, $3, $4)
        "#,
        u.id,
        u.email,
        u.name,
        u.password
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_article(pool: &PgPool, a: Article) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO articles (title, author_id, content)
        VALUES ($1, $2, $3)
        "#,
        a.title,
        a.author.id,  // Now we're using the author’s id
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
        SET title = $1, author_id = $2, content = $3
        WHERE id = $4
        "#,
        a.title,
        a.author.id,  // Now we're using the author's id (author_id)
        a.content,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

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

pub async fn get_article_by_id(pool: &PgPool, id: i32) -> Result<Article, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT a.id, a.title, a.content, u.id as user_id, u.email, u.name, u.password
        FROM articles a
        JOIN users u ON a.author_id = u.id
        WHERE a.id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let article = Article {
        id: row.id,
        title: row.title,
        content: row.content,
        author: User {
            id: row.user_id,
            email: row.email,
            name: row.name,
            password: row.password,
        }
    };

    Ok(article)
}

