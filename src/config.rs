use std::{env, str::FromStr};

/// this file/struct contains environment variables for the app.
/// the goal is to provide structure, so it's easier to know
/// what variables are required, as well as centralize how they're provided
use dotenv::dotenv;

use crate::errors::Errors;

#[derive(Debug)]
pub struct Config {
    pub db_url: String,
    pub db_token: Option<String>,
    pub stage: Stage,
    pub log_level: log::Level,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stage {
    Local,
    Test,
    Prod,
}

impl FromStr for Stage {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Stage::Local),
            "test" => Ok(Stage::Test),
            "prod" => Ok(Stage::Prod),
            _ => Err(Errors::StageParseError),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect("error loading dotenv");
        Config {
            db_url: env::var("DB_URL").expect("error loading env.DB_URL"),

            db_token: Some(env::var("DB_TOKEN").expect("error loading env.DB_TOKEN")),

            stage: env::var("STAGE")
                .expect("error loading env.STAGE")
                .parse()
                .expect("error parsing env.STAGE (does it match one of crate::config::Stage?)"),

            log_level: env::var("LOG_LEVEL")
                .unwrap_or(log::Level::Info.to_string())
                .parse()
                .expect("error parsing env.LOG_LEVEL"),
        }
    }
}
