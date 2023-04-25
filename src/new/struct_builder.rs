use std::collections::BTreeMap;

use crate::new::statements::add_rust_line;

use super::statements::Attribute;

type Key = String;
type Type = String;
pub struct StructBuilder {
    name: String,
    fields: BTreeMap<Key, Type>,
    attr: Option<Attribute>,
    is_pub: bool,
    derives: Vec<String>,
}

impl StructBuilder {
    const SPACE: &'static str = "    ";
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: BTreeMap::new(),
            attr: None,
            is_pub: false,
            derives: Vec::new(),
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
    fn add_field(mut self, key: impl Into<Key>, type_: impl Into<Type>) -> Self {
        self.fields.insert(key.into(), type_.into());
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
        self.fields.iter().fold(String::new(), |acc, (key, value)| {
            add_rust_line(&acc, &format!("{}: {},", key, value))
        })
    }
}

#[derive(Debug)]
struct Filed {
    attr: Option<String>,
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
