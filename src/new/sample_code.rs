struct MainBuilder {
    content: String,
}

impl MainBuilder {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    fn build(self) -> String {
        format!(
            "fn main() {{
    {}}}",
            self.content
        )
    }
    fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}{}\n    ", self.content, line);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            .build();

        assert_eq!(
            main_str,
            "fn main() {\n    let mut map = std::collections::HashMap::new();\n    }"
        );
    }
}
