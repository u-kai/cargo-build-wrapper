#[derive(Debug)]
struct MainBuilder {
    content: String,
}

impl MainBuilder {
    const SPACE: &'static str = "    ";
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    fn build(self) -> String {
        format!(
            "fn main() {{
{}{}}}",
            Self::SPACE,
            self.content
        )
    }
    fn add_line(mut self, line: &str) -> Self {
        self.content = format!("{}{}\n{}", self.content, line, Self::SPACE);
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
