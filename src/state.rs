use std::{env, sync::Arc};
use tera::Tera;
use webauthn_rs::{prelude::Url, Webauthn, WebauthnBuilder};

use crate::Error;

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

pub fn init_webauthn() -> Result<Webauthn, Error> {
    // Effective domain name.
    let rp_id = "lochstep.mcarthur.in";
    // Url containing the effective domain name
    // MUST include the port number!
    let rp_origin = match Url::parse(format!("https://{}", rp_id).as_str()) {
        Ok(url) => url,
        Err(e) => return Err(Box::new(e)),
    };

    let builder = match WebauthnBuilder::new(rp_id, &rp_origin) {
        Ok(builder) => builder,
        Err(e) => return Err(Box::new(e)),
    };

    // Now, with the builder you can define other options.
    // Set a "nice" relying party name. Has no security properties and
    // may be changed in the future.
    let builder = builder
        .rp_name("Lochstep")
        .allow_any_port(true)
        .append_allowed_origin(&Url::parse("https://lochstep.mcarthur.in").unwrap())
        .append_allowed_origin(&Url::parse("https://lochstep.drewmca.dev").unwrap())
        .append_allowed_origin(&Url::parse("http://localhost").unwrap());

    // Consume the builder and create our webauthn instance.
    let webauthn = match builder.build() {
        Ok(webauthn) => webauthn,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(webauthn)
}

pub fn get_app_port() -> String {
    env::var("PORT").unwrap_or("8080".to_string())
}
