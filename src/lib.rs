use lazy_static::lazy_static;
use serde_json::Value;
use std::env;
use std::sync::{Arc, RwLock};
use structs::I18n;

pub use structs::I18nConfig;

mod structs;

static DIR_KEY: &'static str = "INTL_RS_DIR";

static DEFAULT_LANG_KEY: &'static str = "INTL_RS_LANG";

lazy_static! {
    pub static ref I18N: Arc<RwLock<I18n>> = {
        let dir = env::var(DIR_KEY).unwrap_or("src/languages".to_owned());
        let lang = env::var(DEFAULT_LANG_KEY).unwrap_or("zh_CN".to_owned());
        let inner = I18n::init(dir, lang);
        Arc::new(RwLock::new(inner))
    };
}

///find the optimal locale
pub fn find_optimal_locale<S: Into<String>>(locale: S, fallback: bool) -> Option<String> {
    let borrow = I18N.read().unwrap();
    let locale = locale.into();
    if borrow.inner.contains_key(&locale) {
        Some(locale)
    } else if !fallback {
        None
    } else {
        let locale_scope = locale.split(|c| c == '_' || c == '-').collect::<Vec<_>>()[0];
        let locales: Vec<String> = borrow
            .inner
            .iter()
            .filter_map(|(key, _)| {
                let scope = key.split(|c| c == '_' || c == '-').collect::<Vec<_>>()[0];
                if scope == locale_scope {
                    Some(key.to_owned())
                } else {
                    None
                }
            })
            .collect();
        locales.first().cloned()
    }
}

///locale priority options.locale>current_locale>default_locale
///if result is null,use the null_placeholder
pub fn format_message<S: Into<String>, D: Into<String>>(
    key: S,
    default_message: Option<D>,
    options: Option<I18nConfig>,
) -> String {
    let key = key.into();
    let borrow = I18N.read().unwrap();
    let locale = options
        .as_ref()
        .and_then(|ops| ops.locale.to_owned())
        .unwrap_or({
            borrow
                .current_locale
                .as_ref()
                .unwrap_or(&borrow.default_locale)
                .to_owned()
        });
    let fallback = options
        .as_ref()
        .and_then(|ops| ops.fallback)
        .unwrap_or(borrow.fallback);
    let fallback_message = options
        .as_ref()
        .and_then(|ops| ops.null_placeholder.to_owned())
        .unwrap_or(borrow.null_placeholder.to_owned());
    match find_optimal_locale(locale, fallback) {
        None => default_message
            .map(|m| m.into())
            .unwrap_or(fallback_message),
        Some(locale_key) => {
            let configs: &Value = &borrow.inner[&locale_key];
            match key
                .split('.')
                .fold(configs, |result: &Value, k| &result[&k])
            {
                Value::Null => default_message.map(|m| m.into()).unwrap_or(
                    options
                        .as_ref()
                        .and_then(|ops| ops.null_placeholder.to_owned())
                        .unwrap_or(borrow.null_placeholder.to_owned()),
                ),
                other => other
                    .as_str()
                    .map(|other_str| other_str.to_string())
                    .unwrap_or(other.to_string()),
            }
        }
    }
}

///disable the global fallback config
pub fn disable_fallback() {
    I18N.write().unwrap().fallback = false;
}

///enable the global fallback config
pub fn enable_fallback() {
    I18N.write().unwrap().fallback = true;
}

///set the current locale config
pub fn set_current_locale<S: Into<String>>(current_locale: S) {
    I18N.write().unwrap().current_locale = Some(current_locale.into());
}

///set the null placeholder config
pub fn set_null_placeholder<S: Into<String>>(null_placeholder: S) {
    I18N.write().unwrap().null_placeholder = null_placeholder.into();
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::format_message($key, None as Option<String>, None)
    };
    ($key:expr,configs:$configs:expr) => {
        $crate::format_message($key, None as Option<String>, Some($configs))
    };
    ($key:expr,default:$default_message:expr) => {
        $crate::format_message($key, Some($default_message), None)
    };
    ($key:expr,$default_message:expr,$configs:expr) => {
        $crate::format_message($key, Some($default_message), $configs)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i18n_can_find_optimal_locale() {
        env::set_var("INTL_RS_DIR", "languages");
        let key = "en_UK";
        assert_eq!(find_optimal_locale(key, true), Some("en_US".to_owned()));
        assert_eq!(find_optimal_locale(key, false), None);
    }

    #[test]
    fn i18n_can_format_messages() {
        env::set_var("INTL_RS_DIR", "languages");
        let key = "hello.world";
        assert_eq!(t!(key), "你好，世界！");

        assert_eq!(
            t!("unknown key", default:"default message"),
            "default message"
        );

        //default to ensure fallback
        //and you can disable it by disable_fallback function
        let configs = I18nConfig {
            fallback: None,
            locale: Some("en".to_owned()),
            null_placeholder: None,
        };
        assert_eq!(t!(key, configs: configs), "Hello,World!");

        let configs = I18nConfig {
            fallback: Some(true),
            locale: Some("en_UK".to_owned()),
            null_placeholder: None,
        };
        assert_eq!(t!(key, configs: configs), "Hello,World!");

        //change the default null placeholder
        let configs = I18nConfig {
            fallback: Some(true),
            locale: Some("en_UK".to_owned()),
            null_placeholder: Some("".to_owned()),
        };
        assert_eq!(t!("unknown key", configs: configs), "");
    }
}
