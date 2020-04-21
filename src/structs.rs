use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Default)]
pub struct I18nConfig<'a> {
    pub locale: Option<String>,
    pub fallback: Option<bool>,
    pub null_placeholder: Option<String>,
    pub args: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Clone, Debug)]
pub struct I18n {
    pub supported_locales: Vec<String>,
    pub null_placeholder: String,
    pub inner: HashMap<String, Value>,
    ///specific locale>>current locale>>default locale
    pub current_locale: Option<String>,
    pub default_locale: String,
    pub fallback: bool,
}

impl I18n {
    ///init the I18n singleton
    ///it will enumerate the {language}.json under the dir,and store them as language:configs in the
    ///HashMap,so that you can later get them by the language and scope
    pub fn init(dir: String, default_locale: String) -> Self {
        let read_dir = fs::read_dir(&dir).expect("Fail to initialize with current directory");
        let supported_locales: Vec<String> = read_dir
            .filter_map(|entry| {
                entry.ok().and_then(|dir_entry| {
                    dir_entry
                        .path()
                        //just use the file stem,key of en_US.json is en_US
                        .file_stem()
                        .and_then(|file_name| {
                            file_name
                                .to_str()
                                .map(|file_name_str| String::from(file_name_str))
                        })
                })
            })
            .collect();
        let inner: HashMap<String, Value> = supported_locales
            .iter()
            .filter_map(|locale| {
                let file_name = format!("{}/{}.json", &dir, locale);
                fs::read_to_string(file_name).ok().and_then(|content| {
                    serde_json::from_str(content.as_str())
                        .and_then(|value| Ok((locale.clone(), value)))
                        .ok()
                })
            })
            .collect();
        I18n {
            supported_locales,
            inner,
            current_locale: None,
            default_locale,
            null_placeholder: "null".to_owned(),
            fallback: true,
        }
    }
}
