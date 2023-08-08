use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AzdFromKvConfig {
    pub address: String,
}
impl AzdFromKvConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let config = envy::prefixed("AzdFromKvConfig_").from_env::<AzdFromKvConfig>()?;
        Ok(config)
    }
}
