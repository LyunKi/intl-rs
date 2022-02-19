use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Default)]
pub struct TranslationConfig<'a> {
    pub accept_language: Option<&'a str>,
    pub default_message: Option<&'a str>,
    pub args: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Clone, Debug)]
pub struct I18n {
    pub supported_languages: Vec<String>,
    pub inner: HashMap<String, Value>,
    pub default_language: String,
}

impl I18n {
    ///init the I18n singleton
    pub fn init(resources_path: String, default_language: String) -> Self {
        let read_dir =
            fs::read_dir(&resources_path).expect("Fail to initialize with current directory");
        let supported_languages: Vec<String> = read_dir
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
        let inner: HashMap<String, Value> = supported_languages
            .iter()
            .filter_map(|language| {
                let file_name = format!("{}/{}.json", &resources_path, language);
                fs::read_to_string(file_name).ok().and_then(|content| {
                    serde_json::from_str(content.as_str())
                        .and_then(|value| Ok((language.clone(), value)))
                        .ok()
                })
            })
            .collect();
        I18n {
            supported_languages,
            inner,
            default_language,
        }
    }
}
