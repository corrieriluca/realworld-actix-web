//! This module interacts primarily with the "users" database.

use sqlx::PgPool;

use crate::{
    domain::users::{NewUser, UpdateUser},
    models::users::{User, UserWithPassword},
};

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

/// Returns a user by its username. Returns an error if the user does not exist.
pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, bio, image
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Returns a user with its password by searching it with its email.
/// Returns an error if the user does not exist.
pub async fn get_user_with_password_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<UserWithPassword, sqlx::Error> {
    let user = sqlx::query_as!(
        UserWithPassword,
        r#"
        SELECT username, email, password, bio, image
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update the values of an user given its current username.
/// Return the username of the user affected (may be new).
pub async fn update_user(
    pool: &PgPool,
    username: &str,
    updated: &UpdateUser,
) -> Result<String, sqlx::Error> {
    // Generate the `SET ...` string
    let mut properties_to_set = Vec::new();
    if let Some(updated_username) = &updated.username {
        properties_to_set.push(format!("username = '{}'", updated_username.as_ref()));
    }
    if let Some(updated_email) = &updated.email {
        properties_to_set.push(format!("email = '{}'", updated_email.as_ref()));
    }
    if let Some(updated_password) = &updated.password {
        properties_to_set.push(format!("password = '{updated_password}'"));
    }
    if let Some(updated_bio) = &updated.bio {
        properties_to_set.push(format!("bio = '{updated_bio}'"));
    }
    if let Some(updated_image) = &updated.image {
        properties_to_set.push(format!("image = '{updated_image}'"));
    }
    let properties_to_set = properties_to_set.join(",");

    sqlx::query(&format!(
        "UPDATE users SET {} WHERE username = $1",
        properties_to_set
    ))
    .bind(username)
    .execute(pool)
    .await?;

    Ok(if let Some(new_username) = &updated.username {
        new_username.as_ref().into()
    } else {
        username.into()
    })
}
