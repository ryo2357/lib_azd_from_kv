use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Kv7500Config {
    pub address: String,
}
impl Kv7500Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let config = envy::prefixed("Kv7500Config_").from_env::<Kv7500Config>()?;
        Ok(config)
    }
}
