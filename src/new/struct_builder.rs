use crate::new::statements::add_rust_line;

use super::statements::{Attribute, Derive, IntoAttr};

type Key = String;
type Type = String;
pub struct StructBuilder {
    name: String,
    fields: Vec<Filed>,
    attr: Option<Attribute>,
    is_pub: bool,
    is_enum: bool,
    derives: Derive,
}

impl StructBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            is_enum: false,
            name: name.into(),
            fields: Vec::new(),
            attr: None,
            is_pub: false,
            derives: Derive::new(),
        }
    }
    pub fn new_enum(name: impl Into<String>) -> Self {
        Self {
            is_enum: true,
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
        if self.is_enum {
            self.fields.push(Filed::new_enum(key, type_));
            self
        } else {
            self.fields.push(Filed::new(key, type_));
            self
        }
    }
    pub fn add_field_with_attr(
        mut self,
        key: impl Into<Key>,
        type_: impl Into<Type>,
        attr: impl IntoAttr,
    ) -> Self {
        let mut filed = Filed::new(key, type_);
        filed.attr = Some(attr.into_attr());
        self.fields.push(filed);
        self
    }
    pub fn add_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.add(derive);
        self
    }
    fn create_prefix(&self) -> String {
        let prefix = if self.is_enum { "enum" } else { "struct" };
        if self.is_pub {
            format!("pub {} {}", prefix, self.name)
        } else {
            format!("{} {}", prefix, self.name)
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
    is_enum: bool,
    attr: Option<Attribute>,
    key: Key,
    type_: Type,
}
impl Filed {
    fn new(key: impl Into<Key>, type_: impl Into<Type>) -> Self {
        Self {
            is_enum: false,
            attr: None,
            key: key.into(),
            type_: type_.into(),
        }
    }
    fn new_enum(key: impl Into<Key>, type_: impl Into<Type>) -> Self {
        Self {
            is_enum: true,
            attr: None,
            key: key.into(),
            type_: type_.into(),
        }
    }
    fn create_fields(&self) -> String {
        if self.is_enum {
            if self.type_.len() > 0 {
                add_rust_line(
                    &self.create_attr(),
                    &format!("{}({}),", self.key, self.type_),
                )
            } else {
                add_rust_line(&self.create_attr(), &format!("{},", self.key))
            }
        } else {
            add_rust_line(
                &self.create_attr(),
                &format!("{}: {},", self.key, self.type_),
            )
        }
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
    fn 構造体のフィールドにattrを追加できる() {
        let result = StructBuilder::new("Cli")
            .add_field_with_attr("key", "String", "clap(subcommand)")
            .add_field("key2", "usize")
            .build();

        assert_eq!(
            result,
            r#"struct Cli {
    #[clap(subcommand)]
    key: String,
    key2: usize,
}"#
        )
    }
    #[test]
    fn 構造体のフィールドを宣言できる() {
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
    fn enumのフィールドを型なしで生成することができる() {
        let result = StructBuilder::new_enum("Cli").add_field("Key", "").build();

        assert_eq!(
            result,
            r#"enum Cli {
    Key,
}"#
        )
    }
    #[test]
    fn enumを生成することができる() {
        let result = StructBuilder::new_enum("Cli").build();

        assert_eq!(
            result,
            r#"enum Cli {
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
