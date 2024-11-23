use std::env;

use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Environment {
    // debug_assertion が on の場合はデバッグビルド
    // そうでない場合はリリースビルドであると判定する
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(env) => env.parse().unwrap_or(default_env),
    }
}
