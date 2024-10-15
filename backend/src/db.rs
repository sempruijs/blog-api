use crate::User;
use sqlx::PgPool;

//// Inserts a new user into the users database
pub async fn create_user(pool: &PgPool, u: User) -> Result<(), sqlx::Error> {
    async fn update_user(pool: &PgPool, id: i32, u: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET name = $1, age = $2, email = $3
            WHERE id = $4
            "#,
            u.name,
            u.age,
            u.email,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
    sqlx::query!(
        r#"
        INSERT INTO users (name, age, email)
        VALUES ($1, $2, $3)
        "#,
        u.name,
        u.age,
        u.email
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// List all users from the database
pub async fn read_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, r#"SELECT name, age, email FROM users"#)
        .fetch_all(pool)
        .await?;

    Ok(users)
}

/// Update data of a user in the database
pub async fn update_user(pool: &PgPool, id: i32, u: &User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, age = $2, email = $3
        WHERE id = $4
        "#,
        u.name,
        u.age,
        u.email,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Removes an user from the database
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        println!("No user found with id {}", id);
    }

    Ok(())
}
