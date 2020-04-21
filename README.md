[中文文档](./README-zh.md)

A simple i18n library for rust

### Configs

1. The default language files is under`src/languages`,and you could change it by the env var `INTL_RS_DIR`
2. The default locale(default_locale property) is `zh_CN`,
   and you could change it by the env var `INTL_RS_LANG`

### Config File

Just support the json file for example `en_US.json` like below

```json
{
    "hello": {
        "world": "Hello,World!"
    }
}
```

### Usages

```rust
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
```
