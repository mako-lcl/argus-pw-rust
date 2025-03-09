use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_digits: bool,
    pub use_special: bool,
    pub uppercase: String,
    pub lowercase: String,
    pub digits: String,
    pub special: String,
}

// Provide default values for the configuration.
impl Default for Config {
    fn default() -> Self {
        Self {
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_special: true,
            uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
            lowercase: "abcdefghijklmnopqrstuvwxyz".into(),
            digits: "0123456789".into(),
            special: "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".into(),
        }
    }
}
