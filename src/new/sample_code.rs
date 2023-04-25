#[derive(Debug)]
pub struct MainBuilder {
    content: String,
    async_mode: bool,
}

impl MainBuilder {
    const SPACE: &'static str = "    ";
    pub fn new() -> Self {
        Self {
            content: String::new(),
            async_mode: false,
        }
    }
    pub fn async_mode(self) -> Self {
        Self {
            content: self.content,
            async_mode: true,
        }
    }
    pub fn build(self) -> String {
        format!(
            "{} {{{}
}}",
            self.create_prefix_main(),
            self.content
        )
    }
    pub fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}\n{}{}", self.content, Self::SPACE, line);
        self
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
    fn async_main関数を出力することができる() {
        let main_str = MainBuilder::new().async_mode().build();

        assert_eq!(
            main_str,
            "async fn main() {
}"
        );

        //let main_str = MainBuilder::new()
        //.add_line("let mut map = std::collections::HashMap::new();")
        //.build();

        //assert_eq!(
        //main_str,
        //"fn main() {\n    let mut map = std::collections::HashMap::new();\n    }"
        //);
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
