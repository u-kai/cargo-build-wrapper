use super::statements::add_rust_line;

pub struct InnerCommentBuilder {
    inner: String,
}

impl InnerCommentBuilder {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }
    pub fn add_comment(mut self, comment: &str) -> Self {
        self.inner = add_rust_line(self.inner.as_str(), &format!("// {}", comment));
        Self { inner: self.inner }
    }
    pub fn build(self) -> String {
        self.inner
    }
}
impl Default for InnerCommentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
pub struct OuterCommentBuilder {
    inner: String,
}
impl OuterCommentBuilder {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }
    pub fn add_comment(mut self, comment: &str) -> Self {
        self.inner.push_str(&format!("// {}\n", comment));
        Self { inner: self.inner }
    }
    pub fn build(self) -> String {
        self.inner
    }
}
impl Default for OuterCommentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn 外部向けのコメントを記述できる() {
        let result = OuterCommentBuilder::new()
            .add_comment("test")
            .add_comment("fuga")
            .build();

        assert_eq!(
            result,
            r#"// test
// fuga
"#
        );
    }
    #[test]
    fn 内部向けのコメントを記述できる() {
        let result = InnerCommentBuilder::new()
            .add_comment("test")
            .add_comment("fuga")
            .build();

        assert_eq!(
            result,
            r#"
    // test
    // fuga"#
        )
    }
}
