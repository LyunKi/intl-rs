use inflector::Inflector;
use lazy_static::lazy_static;
use serde_json::Value;
use std::env;
use std::sync::{Arc, RwLock};
use string_template::Template;
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
    if !fallback && !borrow.inner.contains_key(&locale) {
        None
    } else {
        let mut common_result: Option<String> = None;
        let mut similar_result: Option<String> = None;
        let snake_locale = locale.as_str().to_snake_case();
        let locale_scope = locale.split(|c| c == '_' || c == '-').collect::<Vec<_>>()[0];
        for (key, _) in borrow.inner.iter() {
            let snake_key = key.as_str().to_snake_case();
            //en_US.json en-US.json completely match en_US.json when fallback flag is true;
            if snake_key == snake_locale {
                return Some(key.to_owned());
            }
            let key_infos = key.split(|c| c == '_' || c == '-').collect::<Vec<_>>();
            let scope = key_infos[0];
            if scope != locale_scope {
                continue;
            }
            // en.json is the common match of en_US.json,en_US.json is the similar match of en_UK.json
            if key_infos.len() == 1 {
                common_result = Some(key.to_owned());
            } else {
                similar_result = Some(key.to_owned());
            }
        }
        if common_result.is_some() {
            common_result
        } else {
            if similar_result.is_some() { similar_result }
            else {
                if fallback { Some(borrow.default_locale.clone()) } else { None }
            }
        }
    }
}

///locale priority options.locale>current_locale>default_locale
///if result is null,use the null_placeholder
pub fn format_message<'a, S: Into<String>, D: Into<String>>(
    key: S,
    default_message: Option<D>,
    options: Option<I18nConfig<'a>>,
) -> String {
    let key = key.into();
    let borrow = I18N.read().unwrap();
    let I18nConfig {
        locale,
        fallback,
        null_placeholder,
        args,
    } = options.unwrap_or_default();
    let locale = locale.unwrap_or({
        borrow
            .current_locale
            .as_ref()
            .unwrap_or(&borrow.default_locale)
            .to_owned()
    });
    let fallback = fallback.unwrap_or(borrow.fallback);
    let fallback_message = null_placeholder.unwrap_or(borrow.null_placeholder.to_owned());
    let default_message = default_message
        .map(|m| m.into())
        .unwrap_or(fallback_message);
    let template_string = match find_optimal_locale(locale, fallback) {
        None => default_message,
        Some(locale_key) => {
            let configs: &Value = &borrow.inner[&locale_key];
            match key
                .split('.')
                .fold(configs, |result: &Value, k| &result[&k])
            {
                Value::Null => default_message,
                other => other
                    .as_str()
                    .map(|other_str| other_str.to_string())
                    .unwrap_or(other.to_string()),
            }
        }
    };
    let template = Template::new(&template_string);
    template.render(&args.unwrap_or_default())
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
    ($key:expr,args:$args:expr) => {
        $crate::format_message(
            $key,
            None as Option<String>,
            Some({
                let mut inner = I18nConfig::default();
                inner.args = Some($args);
                inner
            }),
        )
    };
    ($key:expr,default:$default_message:expr,args:$args:expr) => {
        $crate::format_message(
            $key,
            Some($default_message),
            Some({
                let mut inner = I18nConfig::default();
                inner.args = Some($args);
                inner
            }),
        )
    };
    ($key:expr,$default_message:expr,$configs:expr) => {
        $crate::format_message($key, Some($default_message), $configs)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn i18n_can_find_optimal_locale() {
        env::set_var("INTL_RS_DIR", "languages");
        assert_eq!(find_optimal_locale("en_UK", true), Some("en".to_owned()));

        assert_eq!(find_optimal_locale("en-US", true), Some("en_US".to_owned()));

        assert_eq!(
            find_optimal_locale("en_US", false),
            Some("en_US".to_owned())
        );

        assert_eq!(find_optimal_locale("en_UK", false), None);
    }

    #[test]
    fn fallback_to_default_locale() {
        env::set_var("INTL_RS_DIR", "languages");
        // locale fr_FR is not exists, default should be used on fallback
        assert_eq!(find_optimal_locale("fr_FR", true), Some("zh_CN".to_owned()));
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
            args: None,
        };
        assert_eq!(t!(key, configs: configs), "Hello,World!");

        let configs = I18nConfig {
            fallback: Some(true),
            locale: Some("en_UK".to_owned()),
            null_placeholder: None,
            args: None,
        };
        assert_eq!(t!(key, configs: configs), "Hello,World!");

        //change the default null placeholder
        let configs = I18nConfig {
            fallback: Some(true),
            locale: Some("en_UK".to_owned()),
            null_placeholder: Some("".to_owned()),
            args: None,
        };
        assert_eq!(t!("unknown key", configs: configs), "");
        //render template
        let mut args: HashMap<&str, &str> = HashMap::new();
        args.insert("name", "Donald Trump");

        let configs = I18nConfig {
            fallback: Some(true),
            locale: Some("en_UK".to_owned()),
            null_placeholder: Some("".to_owned()),
            args: Some(args.clone()),
        };
        assert_eq!(
            t!("hello.somebody", configs: configs),
            "Hello,Donald Trump!"
        );

        assert_eq!(
            t!("unknown key",default:"Hey,{{name}}!", args: args.clone()),
            "Hey,Donald Trump!"
        );

        let mut args: HashMap<&str, &str> = HashMap::new();
        args.insert("name", "唐纳德·川普");
        assert_eq!(
            t!("hello.somebody", args: args.clone()),
            "你好，唐纳德·川普！"
        );
    }
}
