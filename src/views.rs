use axum::response::Html;
use tera::{Context, Tera};

use crate::controllers::auth::PasskeyRegistrationOptions;

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

pub fn complete_registration_challenge(
    templates: &Tera,
    challenge: PasskeyRegistrationOptions,
) -> Html<String> {
    let serialized_challenge = serde_json::to_string(&challenge).unwrap();
    let mut ctx = Context::new();
    ctx.insert("challenge", &serialized_challenge);
    Html(
        templates
            .render("components/complete-registration.html", &ctx)
            .expect("error rendering template: complete-registration"),
    )
}
