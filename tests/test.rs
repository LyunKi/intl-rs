use intl_rs::*;
use std::{collections::HashMap, env};

#[test]
fn i18n_can_find_optimal_locale() {
    env::set_var("INTL_RS_RESOURCES", "tests/i18n");
    assert_eq!(find_optimal_locale("en_UK", true), Some("en".to_owned()));

    assert_eq!(find_optimal_locale("en-US", true), Some("en_US".to_owned()));

    assert_eq!(
        find_optimal_locale("en_US", false),
        Some("en_US".to_owned())
    );

    assert_eq!(find_optimal_locale("en_UK", false), None);
}

#[test]
fn i18n_can_format_messages() {
    env::set_var("INTL_RS_RESOURCES", "tests/i18n");
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
        language: Some("en".to_owned()),
        null_placeholder: None,
        args: None,
    };
    assert_eq!(t!(key, configs: configs), "Hello,World!");

    let configs = I18nConfig {
        fallback: Some(true),
        language: Some("en_UK".to_owned()),
        null_placeholder: None,
        args: None,
    };
    assert_eq!(t!(key, configs: configs), "Hello,World!");

    //change the default null placeholder
    let configs = I18nConfig {
        fallback: Some(true),
        language: Some("en_UK".to_owned()),
        null_placeholder: Some("".to_owned()),
        args: None,
    };
    assert_eq!(t!("unknown key", configs: configs), "");
    //render template
    let mut args: HashMap<&str, &str> = HashMap::new();
    args.insert("name", "Donald Trump");

    let configs = I18nConfig {
        fallback: Some(true),
        language: Some("en_UK".to_owned()),
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
