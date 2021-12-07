//! This module interacts primarily with the "users" database.

use sqlx::PgPool;

use crate::domain::users::NewUser;

/// Inserts a new valid user in the database. If the user already exists in the
/// database, this operation might fail (same username and/or same email).
pub async fn insert_new_user(pool: &PgPool, user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        "#,
        user.username.as_ref(),
        user.email.as_ref(),
        user.password,
    )
    .execute(pool)
    .await?;

    Ok(())
}
