use crate::{auth::AuthedUser, db::bug::Bug, AppState};
use askama::Template;
use rocket::{get, Responder, State};

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePage {
    _bugs: Vec<Bug>,
}

#[derive(Responder)]
pub enum HomeResponse {
    // GotoSite(Redirect),
    Continue(HomePage),
}

#[get("/")]
pub async fn home_page(_state: &State<AppState>, _user: AuthedUser) -> HomeResponse {
    // let conn = state.db.conn();
    // let sites = get_sites(&conn, user.0.user_id).await;
    let _bugs: Vec<Bug> = vec![];

    HomeResponse::Continue(HomePage { _bugs })
    // HomeResponse::GotoSite(Redirect::to(format!("/site/{}", sites[0].site_id)))
}
