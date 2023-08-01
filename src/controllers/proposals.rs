use axum::response::Html;

use crate::{errors::Errors, models};

pub async fn proposals_homepage() -> Result<Html<String>, Errors> {
    let all_proposals = models::proposals::all_proposals().await;
    Ok(Html("all proposals!".to_string()))
}
