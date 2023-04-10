use std::{
    collections::BTreeMap,
    env::VarError,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use clap::{Parser, Subcommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cw = Cw::parse();
    match cw.sub {
        Sub::Build { release } => {
            let mode = if release {
                BuildMode::Release
            } else {
                BuildMode::Debug
            };
            let wrapper = CargoBuildWrapper::from_env(mode).unwrap();
            wrapper.build()
        }
        Sub::New { cli, name } => {
            let new = CargoNewWrapper::new(name);
            if cli {
                new.create_new_cli_project()
            } else {
                new.create_new_project()
            }
        }
    }
}

#[derive(Parser)]
struct Cw {
    #[clap(subcommand)]
    sub: Sub,
}

#[derive(Subcommand)]
enum Sub {
    Build {
        #[clap(short, long)]
        release: bool,
    },
    New {
        name: String,
        #[clap(short, long)]
        cli: bool,
    },
}

struct CargoNewWrapper {
    name: String,
}
impl CargoNewWrapper {
    const CLAP_VERSION: &'static str = "4.2.1";
    const RUST_EDITION: &'static str = "2021";
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    fn create_new_cli_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_new_project()?;
        let project_root: &Path = self.name.as_ref();
        let cargo_toml_content = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "{}"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = {{ version = "{}", features = ["derive"] }}"#,
            self.name.as_str(),
            Self::RUST_EDITION,
            Self::CLAP_VERSION
        );

        let main_fn = format!(
            r#"// this is create auto
use clap::{{Parser, Subcommand}};
#[derive(Parser)]
struct Cli {{
    #[clap(subcommand)]
    sub: Sub,
}}

#[derive(Subcommand)]
enum Sub {{
    // sub command hear
    //#[clap(short, long)]
        
}}

fn main() {{
    let cli = Cli::parse();
    //match cli  {{
    //    Sub
    //}}
}}
        "#
        );
        write_file(project_root.join("Cargo.toml"), &cargo_toml_content)?;
        write_file(project_root.join("src/main.rs"), &main_fn)?;
        Ok(())
    }
    fn create_new_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        run_command("cargo", &["new", self.name.as_str()])
    }
}

struct CargoTomlContent {
    name: String,
    dependencies: Vec<CargoDepend>,
    edition: RustEdition,
}
impl CargoTomlContent {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dependencies: Vec::new(),
            edition: RustEdition::V2021,
        }
    }
    fn gen(&self) -> String {
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
    fn add_depend(
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

fn run_command(program: &str, commands: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(program);
    cmd.args(commands);

    let child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child process");

    if let Some(stdout) = child.stdout {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            println!("{}", line?);
        }
    };
    if let Some(stderr) = child.stderr {
        let stderr_reader = BufReader::new(stderr);

        for line in stderr_reader.lines() {
            println!("{}", line?);
        }
    };

    Ok(())
}
fn write_file(path: impl AsRef<Path>, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
struct CargoBuildWrapper {
    mode: BuildMode,
    copy_dir: String,
}

impl CargoBuildWrapper {
    const RELEASE_TARGET_DIR: &'static str = "target/release";
    const DEBUG_TARGET_DIR: &'static str = "target/debug";
    fn new(mode: BuildMode, copy_dir: impl Into<String>) -> Self {
        Self {
            mode,
            copy_dir: copy_dir.into(),
        }
    }
    fn from_env(mode: BuildMode) -> Result<Self, VarError> {
        let copy_dir = std::env::var("RUST_BIN_PATH")?;
        Ok(Self::new(mode, copy_dir))
    }
    fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            BuildMode::Release => run_command("cargo", &["build", "--release"])?,
            BuildMode::Debug => run_command("cargo", &["build"])?,
        };
        self.cp_exes()
    }
    fn get_exe_filepaths(&self) -> Vec<PathBuf> {
        match self.mode {
            BuildMode::Release => get_exe_filepaths(Self::RELEASE_TARGET_DIR),
            BuildMode::Debug => get_exe_filepaths(Self::DEBUG_TARGET_DIR),
        }
    }
    fn cp_exes(&self) -> Result<(), Box<dyn std::error::Error>> {
        for path in self
            .get_exe_filepaths()
            .into_iter()
            .filter_map(|path| path.as_os_str().to_str().map(|p| p.to_owned()))
        {
            cp(&path, &self.copy_dir)?;
        }
        Ok(())
    }
}

enum BuildMode {
    Debug,
    Release,
}
fn ls_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    match std::fs::read_dir(dir.as_ref()) {
        Ok(root_dir) => root_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.file_type() {
                Ok(file_type) => Some((file_type, entry.path())),
                Err(_) => None,
            })
            .fold(Vec::new(), |mut acc, (file_type, path)| {
                if file_type.is_dir() {
                    return acc;
                }
                acc.push(path);
                acc
            }),
        Err(e) => {
            println!("{}", e.to_string());
            panic!("not found path = {:?}", dir.as_ref())
        }
    }
}

trait IntoAttrStr {
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
fn get_exe_filepaths(dir: &str) -> Vec<PathBuf> {
    #[cfg(not(target_os = "windows"))]
    fn is_exe_file(path: &PathBuf) -> bool {
        let Some(Some(path))= path.file_name().map(|f|f.to_str()) else {
                return false
            };
        !path.contains(".")
    }
    #[cfg(target_os = "windows")]
    fn is_exe_file(path: &PathBuf) -> bool {
        path.extension()
            .map(|extension| extension.to_str().map(|extension| extension))
            == Some(Some("exe"))
    }
    ls_files(dir).into_iter().filter(is_exe_file).collect()
}
fn cp(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_command("cp", &["-r", from, to])
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
        //sut.add_attr("features", "derive");
        let expect = r#"clap = { version = "3.0.4", features = ["derive"] }"#;

        assert_eq!(sut.gen_statement(), expect);
    }
    #[test]
    fn cargo_toml_dependのversionしかない時の挙動() {
        let sut = CargoDepend::new("clap", "3.0.4");
        let expect = r#"clap = "3.0.4""#;

        assert_eq!(sut.gen_statement(), expect);
    }
    #[test]
    fn for_testからファイルのパスをすべて取得する() {
        let exes = ls_files("for-test");
        assert_eq!(exes.len(), 2);
        assert_eq!(
            exes[0].as_path().as_os_str().to_str().unwrap(),
            "for-test/test.txt"
        );
        assert_eq!(
            exes[1].as_path().as_os_str().to_str().unwrap(),
            "for-test/exe"
        );
    }
    #[test]
    fn targetからexeファイルのパスのみ取得する() {
        let exes = get_exe_filepaths("target/debug");
        assert_eq!(exes.len(), 1);
        assert_eq!(
            exes[0].as_path().as_os_str().to_str().unwrap(),
            "target/debug/cbw"
        );
    }
}
