use db::{init_db, Db};
use libsql::Database;
use rocket::fs::FileServer;
use shuttle_runtime::SecretStore;

mod auth;
mod catchers;
mod db;
mod routes;

pub struct AppState {
    db: Db,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] _store: SecretStore,
    #[shuttle_turso::Turso(addr = "{secrets.TURSO_ADDR}", token = "{secrets.TURSO_TOKEN}")]
    database: Database,
) -> shuttle_rocket::ShuttleRocket {
    dotenv::dotenv().ok();

    let db = init_db(database).await;
    let app = AppState { db };

    let rocket = rocket::build()
        .mount("/", routes::routes())
        .mount("/assets", FileServer::from("assets"))
        .register("/", catchers::catchers())
        .manage(app);

    Ok(rocket.into())
}
