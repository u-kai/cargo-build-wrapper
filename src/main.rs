use std::{
    env::VarError,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

use clap::Parser;

fn main() -> std::io::Result<()> {
    let cbw = Cbw::parse();
    let mode = if cbw.release {
        BuildMode::Release
    } else {
        BuildMode::Debug
    };
    let wrapper = CargoBuildWrapper::from_env(mode).unwrap();
    wrapper.build()
}

#[derive(Debug, Parser)]
struct Cbw {
    #[clap(short, long)]
    release: bool,
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
    fn build(&self) -> std::io::Result<()> {
        let mut command = std::process::Command::new("cargo");
        let result = match self.mode {
            BuildMode::Release => command.args(["build", "--release"]).output()?,
            BuildMode::Debug => command.args(["build"]).output()?,
        };
        if result.status.success() {
            println!("{:#?}", result.stdout);
        } else {
            println!("error cause");
            println!("{:#?}", from_utf8(&result.stderr));
        }
        self.cp_exes()
    }
    fn get_exe_filepaths(&self) -> Vec<PathBuf> {
        match self.mode {
            BuildMode::Release => get_exe_filepaths(Self::RELEASE_TARGET_DIR),
            BuildMode::Debug => get_exe_filepaths(Self::DEBUG_TARGET_DIR),
        }
    }
    fn cp_exes(&self) -> std::io::Result<()> {
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
fn cp(from: &str, to: &str) -> std::io::Result<()> {
    Command::new("cp").args(["-r", from, to]).output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
