use serde::Deserialize;

/// Added configuration options in Rocket.toml
#[derive(Deserialize)]
pub struct Config {
    pub invite_url: String,
}