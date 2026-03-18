use std::str::FromStr;

use clap::Parser;

use config::Cli;

mod config;
mod counter;
mod error;
mod output;
mod process;

#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    load_locale();

    let cli = Cli::parse();

    process::process(&cli);
}

fn load_locale() {
    let env_lang = std::env::var("LANG")
        .ok()
        .and_then(|v| language_tags::LanguageTag::from_str(v.as_str()).ok());
    let sys_locale = sys_locale::get_locale()
        .and_then(|v| language_tags::LanguageTag::from_str(v.as_str()).ok());
    let language_tag = env_lang
        .or(sys_locale)
        .map(|v| v.primary_language().to_owned())
        .unwrap_or("en".to_owned());

    rust_i18n::set_locale(&language_tag);
}
