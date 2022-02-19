use intl_rs::*;
use std::{collections::HashMap, env};

#[test]
fn i18n_can_format_message() {
    env::set_var("INTL_RS_RESOURCES", "tests/i18n");
    let key = "hello.world";
    assert_eq!(t!(key), "Hello,World!");
}

#[test]
fn i18n_can_format_message_with_specify_accep_language() {
    env::set_var("INTL_RS_RESOURCES", "tests/i18n");
    let key = "hello.world";
    assert_eq!(
        t!(key,accept_langauge:"zh-CN,zh;q=0.9,en;q=0.8"),
        "你好，世界！"
    );
}

#[test]
fn i18n_can_format_message_with_args() {
    env::set_var("INTL_RS_RESOURCES", "tests/i18n");
    let key = "hello.somebody";
    assert_eq!(
        t!(key,accept_langauge:"zh;q=0.9,en;q=0.8,en;q=1.0",args:HashMap::from([("name","Lyunki")])),
        "Hello,Lyunki!"
    );
}
