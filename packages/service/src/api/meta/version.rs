use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct Version {
    pub version: String,
}

impl Version {
    pub const VERSION_TEXT: &'static str = env!("CARGO_PKG_VERSION");
}

impl Default for Version {
    fn default() -> Self {
        Self {
            version: Self::VERSION_TEXT.to_string(),
        }
    }
}
