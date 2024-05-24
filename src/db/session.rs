use crate::auth::SessionId;
use super::user::User;
use libsql::Connection;
use uuid::Uuid;

pub struct Session {
    pub session_id: Uuid,
    pub user_id: i64,
}

impl Session {
    pub fn new(session_id: Uuid, user_id: i64) -> Self {
        Session {
            session_id,
            user_id,
        }
    }

    pub fn with_id(session_id: &SessionId) -> Self {
        Session {
            session_id: session_id.0,
            user_id: -1,
        }
    }
}

pub async fn create_session(conn: &Connection, user: &User) -> Session {
    let session_id= uuid::Uuid::new_v4();
    let _ = conn
        .execute(
            "INSERT INTO sessions (session_id, user_id) VALUES (?1, ?2);",
            [session_id.to_string(), user.user_id.to_string()],
        )
        .await
        .expect("error creating session");

    Session::new(session_id, user.user_id)
}
