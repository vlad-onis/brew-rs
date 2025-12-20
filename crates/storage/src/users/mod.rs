use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;
// CREATE TABLE users (
//     id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
//     first_name VARCHAR(50) NOT NULL,
//     last_name VARCHAR(50) NOT NULL,
//     email VARCHAR(100) NOT NULL UNIQUE,
//     password_hash VARCHAR(100) NOT NULL,
//     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
// );
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn insert_user<'e, E>(user: UserRow, connection: E) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let query = sqlx::query(
        r#"
        INSERT INTO users(first_name, last_name, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    );

    let insert_result = query
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(chrono::Utc::now())
        .bind(chrono::Utc::now())
        .execute(connection)
        .await?;

    debug!("Insert result: {:?}", insert_result);

    Ok(())
}

// pub async fn get_user_by_email<'e>(
//     user: UserRow,
//     connection: impl sqlx::PgExecutor<'e>,
// ) -> Result<UserRow, sqlx::Error> {
//     let query = sqlx::query(
//         r#"
//         SELECT * FROM users
//         WHERE email = $1
//         "#,
//     );

//     let user = query.bind(user.email).fetch_one(connection).await?;

//     println!("ID: {:?}", user);

//     Ok(user)
// }

#[cfg(test)]
pub mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_insert_user() {
        let pool = PgPool::connect("postgres://postgres:postgres@0.0.0.0:5432/brewrs")
            .await
            .unwrap();

        let user = UserRow {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "test@test.com".to_string(),
            password_hash: "password".to_string(),
            created_at: None,
            updated_at: None,
        };

        insert_user(user, &pool).await.unwrap();
    }
}
