use serde::Deserialize;

mod emboss_env;
mod emboss_envs;

pub(crate) use emboss_env::*;
pub(crate) use emboss_envs::*;

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
