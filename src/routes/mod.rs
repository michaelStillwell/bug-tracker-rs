use rocket::{routes, Route};

mod login;
mod home;

pub fn routes() -> Vec<Route> {
    routes![
        login::login_page,
        login::login_page_with_session,
        login::actions::login_action,

        home::home_page,
    ]
}
