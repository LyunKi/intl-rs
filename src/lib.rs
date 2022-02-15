use inflector::Inflector;
use lazy_static::lazy_static;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use string_template::Template;
use structs::I18n;

pub use structs::I18nConfig;

mod structs;

static RESOURCES_KEY: &'static str = "INTL_RS_RESOURCES";

static CONFIG_KEY: &'static str = "INTL_RS_CONFIG";

lazy_static! {
    pub static ref I18N: Arc<I18n> = {
        let resources_path = env::var(RESOURCES_KEY).unwrap_or("i18n".to_owned());
        let config_path = env::var(CONFIG_KEY).unwrap_or("i18n.toml".to_owned());
        let inner = I18n::init(resources_path, config_path);
        Arc::new(inner)
    };
}

///find the optimal locale
pub fn find_optimal_locale<S: Into<String>>(accept_langauge: S, fallback: bool) -> Option<String> {
    let borrow = I18N.read().unwrap();
    let locale = accept_langauge.into();
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
            similar_result
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
        language: locale,
        fallback,
        null_placeholder,
        args,
    } = options.unwrap_or_default();
    let locale = locale.unwrap_or(borrow.default_locale.clone());
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
