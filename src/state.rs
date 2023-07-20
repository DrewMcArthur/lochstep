use axum::response::ErrorResponse;
use hyper::StatusCode;
use std::{env, sync::Arc};
use tera::Tera;
use webauthn_rs::{prelude::Url, Webauthn, WebauthnBuilder};

#[derive(Clone)]
pub struct AppState {
    pub webauthn: Arc<Webauthn>,
    pub templates: Tera,
    pub db: Arc<libsql_client::Client>,
}

impl AppState {
    pub fn new(web_authn: Webauthn, db_client: libsql_client::Client, templates: Tera) -> Self {
        Self {
            webauthn: Arc::new(web_authn),
            templates,
            db: Arc::new(db_client),
        }
    }
}

pub fn init_webauthn() -> Result<Webauthn, ErrorResponse> {
    // Effective domain name.
    let rp_id = "lochstep.mcarthur.in";
    // Url containing the effective domain name
    // MUST include the port number!
    let rp_origin = match Url::parse(format!("https://{}:{}", rp_id, get_app_port()).as_str()) {
        Ok(url) => url,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error parsing RP_ID to url: {}", e.to_string()),
            )
                .into())
        }
    };

    let builder = match WebauthnBuilder::new(rp_id, &rp_origin) {
        Ok(builder) => builder,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error creating webauthn builder: {}", e.to_string()),
            )
                .into())
        }
    };

    // Now, with the builder you can define other options.
    // Set a "nice" relying party name. Has no security properties and
    // may be changed in the future.
    let builder = builder.rp_name("Axum Webauthn-rs");

    // Consume the builder and create our webauthn instance.
    let webauthn = match builder.build() {
        Ok(webauthn) => webauthn,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error building webauthn: {}", e),
            )
                .into())
        }
    };
    Ok(webauthn)
}

pub fn get_app_port() -> String {
    env::var("PORT").unwrap_or("8080".to_string())
}
