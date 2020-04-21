[English Document](./README.md)

一个简单的 i18n 库

### 配置

1. 默认配置文件放在`src/languages` 下，可以通过环境变量`INTL_RS_DIR`修改
2. 默认的 locale(default_locale 属性) 是`zh_CN`，可以通过环境变量`INTL_RS_LANG`修改

### 配置文件

只支持 JSON 格式，例如 `en_US.json`

```json
{
    "hello": {
        "world": "Hello,World!",
        "somebody": "Hello,{{name}}!"
    }
}
```

### 用法用例

```rust
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
```
