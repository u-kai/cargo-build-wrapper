use std::collections::BTreeMap;

use super::statements::{add_rust_line, Attribute, IntoAttr};

#[derive(Debug)]
pub struct MainBuilder {
    inner: FnBuilder,
}

impl MainBuilder {
    pub fn new() -> Self {
        Self {
            inner: FnBuilder::new("main"),
        }
    }
    pub fn attr(self, attr: impl Into<String>) -> Self {
        Self {
            inner: self.inner.attr(attr),
        }
    }
    pub fn retu(self, retu: impl Into<String>) -> Self {
        Self {
            inner: self.inner.retu(retu),
        }
    }
    pub fn async_mode(self) -> Self {
        Self {
            inner: self.inner.async_mode(),
        }
    }
    pub fn build(self) -> String {
        self.inner.build()
    }
    pub fn add_line(self, line: &str) -> Self {
        Self {
            inner: self.inner.add_line(line),
        }
    }
}

type Arg = String;
type Type = String;
#[derive(Debug)]
pub struct FnBuilder {
    name: String,
    content: String,
    args: BTreeMap<Arg, Type>,
    retu: Option<Type>,
    attr: Option<Attribute>,
    async_mode: bool,
}

impl FnBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: String::new(),
            attr: None,
            async_mode: false,
            args: BTreeMap::new(),
            retu: None,
        }
    }
    pub fn attr(self, attr: impl Into<String>) -> Self {
        Self {
            attr: Some(attr.into_attr()),
            ..self
        }
    }
    pub fn async_mode(self) -> Self {
        Self {
            async_mode: true,
            ..self
        }
    }
    pub fn build(self) -> String {
        format!(
            "{}{}{} {{{}
}}",
            self.create_attr(),
            self.create_prefix_fn(),
            self.create_return(),
            self.content
        )
    }
    pub fn add_arg(mut self, key: impl Into<Arg>, type_: impl Into<Type>) -> Self {
        self.args.insert(key.into(), type_.into());
        self
    }
    pub fn retu(mut self, return_type: impl Into<Type>) -> Self {
        self.retu = Some(return_type.into());
        self
    }
    pub fn add_line(mut self, line: &str) -> Self {
        self.content = add_rust_line(&self.content, line);
        self
    }
    fn create_return(&self) -> String {
        self.retu
            .as_ref()
            .map(|s| format!(" -> {}", s))
            .unwrap_or_default()
    }
    fn create_attr(&self) -> String {
        self.attr
            .as_ref()
            .map(|attr| format!("{}\n", attr.to_string()))
            .unwrap_or_default()
    }
    fn create_prefix_fn(&self) -> String {
        if self.async_mode {
            format!("async fn {}({})", self.name, self.create_args())
        } else {
            format!("fn {}({})", self.name, self.create_args())
        }
    }
    fn create_args(&self) -> String {
        self.args.iter().fold(String::new(), |acc, (key, value)| {
            format!("{}{}: {},", acc, key, value)
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn integration_test() {
        let main_str = FnBuilder::new("main")
            .attr("actix_web::main")
            .async_mode()
            .add_line("HttpServer::new(|| App::new().service(hello).service(echo))")
            .add_line(r#"    .bind(("127.0.0.1", 8080))?"#)
            .add_line(r#"    .run()"#)
            .add_line(r#"    .await"#)
            .build();
        assert_eq!(
            main_str,
            r#"#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}"#
        );
    }
    #[test]
    fn 関数に返り値を与えることができる() {
        let main_str = FnBuilder::new("test").retu("String").build();
        assert_eq!(
            main_str,
            "fn test() -> String {
}"
        );
    }
    #[test]
    fn 関数に引数を与えることができる() {
        let main_str = FnBuilder::new("test")
            .add_arg("key", "String")
            .add_arg("key2", "usize")
            .build();
        assert_eq!(
            main_str,
            "fn test(key: String,key2: usize,) {
}"
        );
    }
    #[test]
    fn main関数に属性を付与することができる() {
        let main_str = MainBuilder::new().attr("tokio::main").build();
        assert_eq!(
            main_str,
            "#[tokio::main]
fn main() {
}"
        );
    }
    #[test]
    fn async_main関数を出力することができる() {
        let main_str = MainBuilder::new().async_mode().build();

        assert_eq!(
            main_str,
            "async fn main() {
}"
        );
    }
    #[test]
    fn main関数を出力することができる() {
        let main_str = MainBuilder::new().build();

        assert_eq!(
            main_str,
            "fn main() {
}"
        );

        let main_str = MainBuilder::new()
            .add_line("let mut map = std::collections::HashMap::new();")
            .add_line(r#"map.insert("key","value");"#)
            .build();

        assert_eq!(
            main_str,
            r#"fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert("key","value");
}"#
        );
    }
}
