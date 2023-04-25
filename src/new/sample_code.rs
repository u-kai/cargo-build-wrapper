use std::collections::HashMap;

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
struct FnBuilder {
    name: String,
    content: String,
    args: HashMap<Arg, Type>,
    retr: Option<Type>,
    attr: Option<String>,
    async_mode: bool,
}

impl FnBuilder {
    const SPACE: &'static str = "    ";
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: String::new(),
            attr: None,
            async_mode: false,
            args: HashMap::new(),
            retr: None,
        }
    }
    pub fn attr(self, attr: impl Into<String>) -> Self {
        Self {
            attr: Some(attr.into()),
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
    pub fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}\n{}{}", self.content, Self::SPACE, line);
        self
    }
    fn create_return(&self) -> String {
        self.retr
            .as_ref()
            .map(|s| format!(" -> {}", s))
            .unwrap_or_default()
    }
    fn create_attr(&self) -> String {
        self.attr
            .as_ref()
            .map(|s| format!("#[{}]\n", s))
            .unwrap_or_default()
    }
    fn create_prefix_fn(&self) -> String {
        if self.async_mode {
            format!("async fn {}()", self.name)
        } else {
            format!("fn {}()", self.name)
        }
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
