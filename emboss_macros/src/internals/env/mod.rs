use serde::Deserialize;

mod emboss_env;
mod emboss_envs;

pub use emboss_env::*;
pub use emboss_envs::*;


#[derive(Deserialize)]
enum EnvVarFallback {
    Fail,
    Empty,
    Value(String),
}

impl Default for EnvVarFallback {
    fn default() -> Self {
        Self::Fail
    }
}
