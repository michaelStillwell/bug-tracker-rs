use rocket::{catch, catchers, http::Status, response::Redirect, Catcher, Request};

#[catch(default)]
pub fn def(status: Status, req: &Request) -> Redirect {
    println!("you are on the default, {}: {}", status, req.uri());
    Redirect::to("/")
}

#[catch(404)]
pub fn not_found() {
    println!("you are on the not_found");
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Redirect {
    let path = req.uri().path();
    Redirect::to(if !path.starts_with("/login") {
        format!("/login?redirect_url={path}")
    } else {
        "/login".to_string()
    })
}

pub fn catchers() -> Vec<Catcher> {
    catchers![unauthorized, def, not_found]
}
