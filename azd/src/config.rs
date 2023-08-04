use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AzdUdpConfig {
    pub address: String,
}
impl AzdUdpConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let config = envy::prefixed("AzdUdpConfig_").from_env::<AzdUdpConfig>()?;
        Ok(config)
    }
}
