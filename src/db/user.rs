use super::session::Session;
use crate::auth::{create_hash, validate_password, Password};
use libsql::{de, Connection, Row};

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub user_id: i64,
    pub email: String,
    pub created_at: String, // TODO: change to Date?
}

impl From<&Row> for User {
    fn from(row: &Row) -> Self {
        de::from_row(row).expect("could not unwrap row for user")
    }
}

#[derive(Clone)]
pub struct InsertUser {
    pub email: String,
    pub password_hash: String,
}

impl InsertUser {
    pub fn new(email: &str, password: &str) -> Self {
        let password = create_hash(password.to_string());

        let email = email.to_string();
        let password_hash = password.hash;

        InsertUser {
            email,
            password_hash,
        }
    }
}

pub async fn get_user_with_login(
    conn: &Connection,
    email: &str,
    password: &str,
) -> anyhow::Result<User> {
    let mut rows = conn
        .query(
            "SELECT user_id, email, password_hash, created_at FROM users 
                WHERE email=?1;",
            [email],
        )
        .await?;

    if let Some(row) = rows.next().await.unwrap() {
        let user = User::from(&row);
        let hash = row.get_str(2).unwrap();
        let valid = validate_password(
            password,
            Password {
                hash: hash.to_string(),
            },
        );

        if valid {
            Ok(user)
        } else {
            Err(anyhow::Error::msg("user not found"))
        }
    } else {
        Err(anyhow::Error::msg("user not found"))
    }
}

pub async fn get_user_with_session(conn: &Connection, session: Session) -> anyhow::Result<User> {
    let mut rows = conn
        .query(
            "SELECT u.user_id, u.email, u.created_at FROM users AS u
                LEFT JOIN sessions AS s USING(user_id)
                WHERE s.session_id=?1;",
            [session.session_id.to_string()],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let user = User::from(&row);

        Ok(user)
    } else {
        Err(anyhow::Error::msg("user not found"))
    }
}
