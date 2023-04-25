#[derive(Debug)]
pub struct MainBuilder {
    content: String,
}

impl MainBuilder {
    const SPACE: &'static str = "    ";
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    pub fn build(self) -> String {
        format!(
            "fn main() {{{}
}}",
            self.content
        )
    }
    pub fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}\n{}{}", self.content, Self::SPACE, line);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn async_main関数を出力することができる() {
        //let main_str = MainBuilder::new().async_mode().build();

        //assert_eq!(
        //main_str,
        //"async fn main() {
        //}"
        //);

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
