use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AzdFromKvConfig {
    pub address: String,
}
impl AzdFromKvConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let config = match envy::prefixed("AzdFromKvConfig_").from_env::<AzdFromKvConfig>() {
            Ok(builder) => builder,
            Err(err) => {
                error!(".envからAzdFromKvConfig_の読み込み失敗：{:?}", err);
                anyhow::bail!(".envからAzdFromKvConfig_の読み込み失敗：{:?}", err)
            }
        };
        Ok(config)
    }
}
