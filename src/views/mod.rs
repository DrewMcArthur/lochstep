use axum::response::Html;
use log::error;
use tera::{Context, Tera};

use crate::{errors::Errors, models::users::User};

pub fn homepage(
    templates: Tera,
    name: String,
    all_users: Vec<User>,
) -> Result<Html<String>, Errors> {
    let mut ctx = Context::new();
    ctx.insert("name", &name);
    ctx.insert("all_users", &all_users);

    match templates.render("homepage.html", &ctx) {
        Ok(html) => Ok(Html(html)),
        Err(e) => Err(Errors::RenderingError("homepage".to_string(), e)),
    }
}

pub fn login(templates: Tera) -> Html<String> {
    Html(
        templates
            .render("login.html", &Context::new())
            .expect("error rendering login page"),
    )
}

pub fn login_success(templates: Tera) -> Result<Html<String>, Errors> {
    let html = match templates.render("login_success.html", &Context::new()) {
        Ok(html) => html,
        Err(e) => {
            error!("error rendering login success result: {}", e);
            return Err(Errors::RenderingError("login_sucess".to_string(), e));
        }
    };
    Ok(Html(html))
}
