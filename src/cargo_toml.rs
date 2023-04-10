use std::collections::BTreeMap;

pub struct CargoTomlContent {
    name: String,
    dependencies: Vec<CargoDepend>,
    edition: RustEdition,
}
impl CargoTomlContent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dependencies: Vec::new(),
            edition: RustEdition::V2021,
        }
    }
    pub fn gen(&self) -> String {
        let expected = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "{}"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]{}"#,
            self.name,
            self.edition.into_str(),
            self.gen_depends()
        );
        expected
    }
    fn gen_depends(&self) -> String {
        self.dependencies.iter().fold(String::new(), |acc, cur| {
            format!("{}\n{}", acc, cur.gen_statement())
        })
    }
    pub fn add_depend(
        &mut self,
        name: impl Into<String>,
        version: impl Into<String>,
        map: (impl Into<String>, impl IntoAttrStr),
    ) {
        let mut depend = CargoDepend::new(name, version);
        depend.add_attr(map.0, map.1);
        self.dependencies.push(depend);
    }
}
struct CargoDepend {
    name: String,
    version: String,
    attr: BTreeMap<String, String>,
}
enum RustEdition {
    V2021,
    V2018,
    V2015,
}
impl RustEdition {
    fn into_str(&self) -> &'static str {
        match self {
            Self::V2015 => "2015",
            Self::V2018 => "2018",
            Self::V2021 => "2021",
        }
    }
}

impl CargoDepend {
    fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            attr: BTreeMap::new(),
        }
    }
    fn gen_statement(&self) -> String {
        if self.attr.len() == 0 {
            format!(r#"{} = "{}""#, self.name, self.version)
        } else {
            let features = self.attr.iter().fold(
                format!(r#"{{ version = "{}""#, self.version),
                |acc, (name, value)| format!(r#"{}, {} = {}"#, acc, name, value),
            );
            format!("{} = {} }}", self.name, features)
        }
    }
    fn add_attr(&mut self, name: impl Into<String>, value: impl IntoAttrStr) {
        self.attr.insert(name.into(), value.into_str());
    }
}

pub trait IntoAttrStr {
    fn into_str(&self) -> String;
}
impl IntoAttrStr for &str {
    fn into_str(&self) -> String {
        self.to_string()
    }
}
impl IntoAttrStr for Vec<&str> {
    fn into_str(&self) -> String {
        let inner = self
            .iter()
            .map(|s| format!(r#""{}""#, s))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{}]", inner)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cargo_toml_content_test() {
        let name = "test";
        let expected = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = {{ version = "3", features = ["derive"] }}"#,
            name
        );
        let mut content = CargoTomlContent::new(name);
        content.add_depend("clap", "3", ("features", vec!["derive"]));
        assert_eq!(content.gen(), expected);
    }
    #[test]
    fn cargo_toml_dependのattrがある時の挙動() {
        let mut sut = CargoDepend::new("clap", "3.0.4");
        sut.add_attr("features", vec!["derive"]);
        let expect = r#"clap = { version = "3.0.4", features = ["derive"] }"#;

        assert_eq!(sut.gen_statement(), expect);
    }
    #[test]
    fn cargo_toml_dependのversionしかない時の挙動() {
        let sut = CargoDepend::new("clap", "3.0.4");
        let expect = r#"clap = "3.0.4""#;

        assert_eq!(sut.gen_statement(), expect);
    }
}
