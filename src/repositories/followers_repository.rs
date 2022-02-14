//! This module interacts primarily with the "followers" table.

use sqlx::PgPool;

/// Returns true if `user1` is following `user2`.
pub async fn is_following(pool: &PgPool, user1: &str, user2: &str) -> Result<bool, sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT *
        FROM followers
        WHERE follower = $1
            AND followed = $2
        "#,
        user1,
        user2
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => Ok(true),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(false),
            x => Err(x),
        },
    }
}
