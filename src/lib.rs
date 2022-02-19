use lazy_static::lazy_static;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use string_template::Template;
use structs::I18n;

pub use structs::TranslationConfig;

mod structs;

static RESOURCES_KEY: &'static str = "INTL_RS_RESOURCES";

static DEFAULT_LANGUAGE: &'static str = "INTL_RS_LANG";

lazy_static! {
    pub static ref I18N: Arc<I18n> = {
        let resources_path = env::var(RESOURCES_KEY).unwrap_or("src/i18n".to_owned());
        let default_language = env::var(DEFAULT_LANGUAGE).unwrap_or("en".to_owned());
        let inner = I18n::init(resources_path, default_language);
        Arc::new(inner)
    };
}

pub fn format_message<'a, S: Into<String>>(
    key: S,
    config: Option<TranslationConfig<'a>>,
) -> String {
    let key = key.into();
    let TranslationConfig {
        accept_language,
        default_message,
        args,
    } = config.unwrap_or_default();
    let common_languages = accept_language::intersection(
        &accept_language.unwrap_or(&I18N.default_language),
        I18N.supported_languages.iter().map(|s| s as &str).collect(),
    );
    let language = common_languages
        .first()
        .expect("No supported language can be found");
    let configs: &Value = &I18N.inner[language];

    let template_string = match key
        .split('.')
        .fold(configs, |result: &Value, k| &result[&k])
    {
        Value::Null => default_message.map(ToString::to_string),
        message => message.as_str().map(|other_str| other_str.to_string()),
    }
    .unwrap_or(key.to_string());
    let template = Template::new(&template_string);
    template.render(&args.unwrap_or_default())
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::format_message($key, None)
    };
    ($key:expr,accept_language:$accept_language:expr) => {
        $crate::format_message(
            $key,
            Some($crate::TranslationConfig {
                accept_language: Some($accept_language),
                ..Default::default()
            }),
        )
    };
    ($key:expr,accept_language:$accept_language:expr,args:$args:expr) => {
        $crate::format_message(
            $key,
            Some($crate::TranslationConfig {
                accept_language: Some($accept_language),
                args: Some($args),
                ..Default::default()
            }),
        )
    };
    ($key:expr,config:$config:expr) => {
        $crate::format_message($key, Some($config))
    };
}
