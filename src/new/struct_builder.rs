use crate::new::statements::add_rust_line;

use super::statements::{Attribute, Derive};

type Key = String;
type Type = String;
pub struct StructBuilder {
    name: String,
    fields: Vec<Filed>,
    attr: Option<Attribute>,
    is_pub: bool,
    derives: Derive,
}

impl StructBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
            attr: None,
            is_pub: false,
            derives: Derive::new(),
        }
    }
    pub fn build(self) -> String {
        format!(
            "{}{}{} {{{}
}}",
            self.create_attr(),
            self.create_derives(),
            self.create_prefix(),
            self.create_fields()
        )
    }
    pub fn add_field(mut self, key: impl Into<Key>, type_: impl Into<Type>) -> Self {
        self.fields.push(Filed::new(key, type_));
        self
    }
    pub fn add_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.add(derive);
        self
    }
    fn create_prefix(&self) -> String {
        if self.is_pub {
            format!("pub struct {}", self.name)
        } else {
            format!("struct {}", self.name)
        }
    }
    fn create_attr(&self) -> String {
        self.attr
            .as_ref()
            .map(|attr| attr.to_string())
            .unwrap_or_default()
    }
    fn create_derives(&self) -> String {
        String::new()
    }
    fn create_fields(&self) -> String {
        self.fields.iter().fold(String::new(), |acc, filed| {
            format!("{}{}", &acc, &filed.create_fields())
        })
    }
}

#[derive(Debug)]
struct Filed {
    attr: Option<String>,
    key: Key,
    type_: Type,
}
impl Filed {
    fn new(key: impl Into<Key>, type_: impl Into<Type>) -> Self {
        Self {
            attr: None,
            key: key.into(),
            type_: type_.into(),
        }
    }
    fn create_fields(&self) -> String {
        add_rust_line(
            &self.create_attr(),
            &format!("{}: {},", self.key, self.type_),
        )
    }
    fn create_attr(&self) -> String {
        self.attr
            .as_ref()
            .map(|attr| add_rust_line("", &attr.to_string()))
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn 構造体をにフィールドを宣言できる() {
        let result = StructBuilder::new("Cli")
            .add_field("key", "String")
            .add_field("key2", "usize")
            .build();

        assert_eq!(
            result,
            r#"struct Cli {
    key: String,
    key2: usize,
}"#
        )
    }
    #[test]
    fn 構造体を生成することができる() {
        let result = StructBuilder::new("Cli").build();

        assert_eq!(
            result,
            r#"struct Cli {
}"#
        )
    }
}
