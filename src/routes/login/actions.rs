use crate::{
    auth::{set_session, SessionId},
    db::{session::create_session, user::get_user_with_login},
    AppState,
};
use askama::Template;
use htmx_rocket::HxHeader;
use rocket::{form::Form, http::CookieJar, post, FromForm, Responder, State};

#[derive(Template)]
#[template(source = "<p class=\"error\">{{ text }}</p>", ext = "html")]
pub struct ErrorText {
    text: String,
}

#[derive(Responder)]
pub enum Response {
    Success(HxHeader),
    Failure(ErrorText),
}

#[derive(FromForm)]
pub struct LoginActionForm {
    username: String,
    password: String,
}

#[post("/login?<redirect_url>", data = "<form>")]
pub async fn login_action(
    redirect_url: Option<String>,
    form: Form<LoginActionForm>,
    state: &State<AppState>,
    jar: &CookieJar<'_>,
) -> Response {
    let conn = state.db.conn();
    if let Ok(user) = get_user_with_login(&conn, &form.username, &form.password).await {
        let session = create_session(&conn, &user).await;
        set_session(jar, SessionId(session.session_id));
        Response::Success(HxHeader::Redirect(redirect_url.unwrap_or("/".to_string())))
    } else {
        Response::Failure(ErrorText {
            text: "Invalid login".to_string(),
        })
    }
}
