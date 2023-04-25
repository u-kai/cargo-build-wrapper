#[derive(Debug)]
pub struct MainBuilder {
    content: String,
    attr: Option<String>,
    async_mode: bool,
}

impl MainBuilder {
    const SPACE: &'static str = "    ";
    pub fn new() -> Self {
        Self {
            content: String::new(),
            attr: None,
            async_mode: false,
        }
    }
    pub fn attr(self, attr: impl Into<String>) -> Self {
        Self {
            content: self.content,
            attr: Some(attr.into()),
            async_mode: self.async_mode,
        }
    }
    pub fn async_mode(self) -> Self {
        Self {
            content: self.content,
            attr: self.attr,
            async_mode: true,
        }
    }
    pub fn build(self) -> String {
        format!(
            "{}{} {{{}
}}",
            self.create_attr(),
            self.create_prefix_main(),
            self.content
        )
    }
    pub fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}\n{}{}", self.content, Self::SPACE, line);
        self
    }
    fn create_attr(&self) -> String {
        self.attr
            .as_ref()
            .map(|s| format!("#[{}]\n", s))
            .unwrap_or_default()
    }
    fn create_prefix_main(&self) -> String {
        if self.async_mode {
            String::from("async fn main()")
        } else {
            String::from("fn main()")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
