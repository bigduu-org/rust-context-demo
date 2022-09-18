use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConfigurationProperties {
    pub(crate) bigduu: BigduuProperties,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BigduuProperties {
    pub(crate) log4rs: Log4rsProperties,
    pub(crate) tokio: TokioProperties,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Log4rsProperties {
    pub(crate) resources: String,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Default)]
pub struct TokioProperties {
    pub(crate) workers: i32,
}
