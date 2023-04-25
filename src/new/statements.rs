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
    pub fn to_string(self) -> String {
        if self.values.len() > 0 {
            self.values
                .into_iter()
                .map(|s| format!("#[{}]\n", s))
                .reduce(|acc, s| format!("{}{}", acc, s))
                .unwrap()
        } else {
            String::new()
        }
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
                "#[derive({})]\n",
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
        assert_eq!(attr.to_string(), "#[derive(Debug)]\n");
        let mut attr = Derive::new();
        attr.add("Debug");
        attr.add("Clone");
        assert_eq!(attr.to_string(), "#[derive(Debug,Clone)]\n")
    }
    #[test]
    fn attr_to_string_test() {
        let mut attr = Attribute::new();
        attr.add("cfg(test)");
        assert_eq!(attr.to_string(), "#[cfg(test)]\n");
        let mut attr = Attribute::new();
        attr.add("cfg(test)");
        attr.add("cfg(target=windows)");
        assert_eq!(attr.to_string(), "#[cfg(test)]\n#[cfg(target=windows)]\n")
    }
}
