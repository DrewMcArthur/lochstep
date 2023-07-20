use axum::response::Html;
use log::error;
use tera::{Context, Tera};

use crate::errors::Errors;

pub fn homepage(templates: Tera) -> Html<String> {
    Html(
        templates
            .render("homepage.html", &Context::new())
            .expect("error rendering homepage"),
    )
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
