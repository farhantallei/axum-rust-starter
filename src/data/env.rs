use dotenvy::dotenv;
use envforge::{CreateEnvOpts, create_env};
use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};
use std::collections::HashMap;
use std::env;
use validator::Validate;

#[serde_as]
#[derive(Debug, Deserialize, Validate)]
pub struct Env {
    #[serde(rename = "PORT")]
    #[serde_as(as = "DisplayFromStr")]
    pub port: u16,

    #[serde(rename = "DATABASE_URL")]
    #[serde_as(as = "DisplayFromStr")]
    pub database_url: String,
}

pub fn init_env() -> Env {
    dotenv().ok();

    let runtime_env = env::vars().collect::<HashMap<_, _>>();

    create_env(CreateEnvOpts {
        runtime_env,
        empty_string_as_undefined: true,
    })
}
