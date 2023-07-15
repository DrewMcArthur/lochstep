use axum::response::Html;
use serde::{Serialize, Serializer};
use tera::{Context, Tera};

use crate::controllers::auth::BeginRegistrationResponse;

pub fn render_homepage(templates: Tera, logged_in: &bool) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("authenticated", logged_in);
    Html(
        templates
            .render("homepage.html", &ctx)
            .expect("error rendering homepage"),
    )
}

pub fn complete_registration_challenge(
    templates: &Tera,
    challenge: BeginRegistrationResponse,
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
