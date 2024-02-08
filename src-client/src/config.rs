use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum ConfigSerialized {
    #[serde(rename = "v1")]
    V1(ConfigV1),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigV1 {
    remote_url: String,
    // TODO: Authentication
}

#[derive(Clone)]
pub struct Config {
    pub remote_url: String,
}

impl From<ConfigSerialized> for Config {
    fn from(value: ConfigSerialized) -> Self {
        match value {
            ConfigSerialized::V1(ConfigV1 { remote_url }) => Config { remote_url },
        }
    }
}

impl From<Config> for ConfigSerialized {
    fn from(value: Config) -> Self {
        let Config { remote_url } = value;
        ConfigSerialized::V1(ConfigV1 { remote_url })
    }
}
