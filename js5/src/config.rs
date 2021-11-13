use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub revision: i32,
    pub port: Option<u16>,
}
