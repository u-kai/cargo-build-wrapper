use std::{env::VarError, path::PathBuf};

use crate::commands::{cp, get_exe_filepaths, run_command};

pub struct CargoBuildWrapper {
    mode: BuildMode,
    copy_dir: String,
}

impl CargoBuildWrapper {
    const RELEASE_TARGET_DIR: &'static str = "target/release";
    const DEBUG_TARGET_DIR: &'static str = "target/debug";
    pub fn new(mode: BuildMode, copy_dir: impl Into<String>) -> Self {
        Self {
            mode,
            copy_dir: copy_dir.into(),
        }
    }
    pub fn from_env(mode: BuildMode) -> Result<Self, VarError> {
        let copy_dir = std::env::var("RUST_BIN_PATH")?;
        Ok(Self::new(mode, copy_dir))
    }
    pub fn build(&self) -> Result<(), Box<dyn std::error::Error>> {
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

pub enum BuildMode {
    Debug,
    Release,
}
