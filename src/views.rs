use axum::response::Html;
use tera::{Context, Tera};

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
