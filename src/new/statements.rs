pub fn add_rust_line(acc: &str, line: &str) -> String {
    format!("{}\n    {}", acc, line)
}
#[derive(Debug)]
pub struct Attribute {
    values: Vec<String>,
}

impl Attribute {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn add(&mut self, value: impl Into<String>) {
        self.values.push(value.into());
    }
    pub fn to_string(&self) -> String {
        if self.values.len() > 0 {
            self.values
                .iter()
                .map(|s| format!("#[{}]", s))
                .reduce(|acc, s| format!("{}\n{}", acc, s))
                .unwrap()
        } else {
            String::new()
        }
    }
}
pub trait IntoAttr {
    fn into_attr(self) -> Attribute;
}
impl<T> IntoAttr for T
where
    T: Into<String>,
{
    fn into_attr(self) -> Attribute {
        let mut result = Attribute::new();
        result.add(self);
        result
    }
}
pub struct Derive {
    values: Vec<String>,
}

impl Derive {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn add(&mut self, value: impl Into<String>) {
        self.values.push(value.into());
    }
    pub fn to_string(self) -> String {
        if self.values.len() > 0 {
            format!(
                "#[derive({})]",
                self.values
                    .into_iter()
                    .reduce(|acc, s| format!("{},{}", acc, s))
                    .unwrap()
            )
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::new::statements::Derive;

    use super::Attribute;

    #[test]
    fn derive_to_string_test() {
        let mut attr = Derive::new();
        attr.add("Debug");
        assert_eq!(attr.to_string(), "#[derive(Debug)]");
        let mut attr = Derive::new();
        attr.add("Debug");
        attr.add("Clone");
        assert_eq!(attr.to_string(), "#[derive(Debug,Clone)]")
    }
    #[test]
    fn attr_to_string_test() {
        let mut attr = Attribute::new();
        attr.add("cfg(test)");
        assert_eq!(attr.to_string(), "#[cfg(test)]");
        let mut attr = Attribute::new();
        attr.add("cfg(test)");
        attr.add("cfg(target=windows)");
        assert_eq!(attr.to_string(), "#[cfg(test)]\n#[cfg(target=windows)]")
    }
}
