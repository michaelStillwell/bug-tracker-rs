use crate::auth::AuthedUser;
use askama::Template;
use rocket::{get, response::Redirect};

pub(super) mod actions;

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginPage {
    redirect_url: Option<String>,
}

#[get("/login?<redirect_url>")]
pub fn login_page_with_session(_user: AuthedUser, redirect_url: Option<String>) -> Redirect {
    let to = if let Some(redirect_url) = redirect_url {
        if redirect_url.starts_with('/') {
            redirect_url
        } else {
            format!("/{redirect_url}")
        }
    } else {
        "/".to_string()
    };

    Redirect::to(to)
}

#[get("/login?<redirect_url>", rank = 2)]
pub fn login_page(redirect_url: Option<String>) -> LoginPage {
    LoginPage { redirect_url }
}
