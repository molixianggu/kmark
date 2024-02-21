use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,
    pub private_key: String,
    pub protocol_id: u64,
}

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Settings = Config::builder()
        .add_source(config::File::with_name("configs/web"))
        .build()
        .unwrap().try_deserialize().unwrap();
}
