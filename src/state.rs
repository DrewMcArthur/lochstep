use std::sync::Arc;
use tera::Tera;
use webauthn_rs::{prelude::Url, Webauthn, WebauthnBuilder};

#[derive(Clone)]
pub struct AppState {
    pub webauthn: Arc<Webauthn>,
    pub templates: Tera,
    pub db: Arc<libsql_client::Client>,
}

impl AppState {
    pub fn new(db_client: libsql_client::Client) -> Self {
        AppState {
            webauthn: Arc::new(init_webauthn()),
            templates: init_templates(),
            db: Arc::new(db_client),
        }
    }
}

fn init_webauthn() -> Webauthn {
    // Effective domain name.
    let rp_id = "localhost";
    // Url containing the effective domain name
    // MUST include the port number!
    let rp_origin = Url::parse("http://localhost:3000").expect("Invalid URL");
    let builder = WebauthnBuilder::new(rp_id, &rp_origin).expect("Invalid configuration");

    // Now, with the builder you can define other options.
    // Set a "nice" relying party name. Has no security properties and
    // may be changed in the future.
    let builder = builder.rp_name("Axum Webauthn-rs");

    // Consume the builder and create our webauthn instance.
    builder.build().expect("Invalid configuration")
}

fn init_templates() -> Tera {
    match Tera::new("src/ui/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}
