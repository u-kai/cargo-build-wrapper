use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn run_command(program: &str, commands: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn ls_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
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

pub fn get_exe_filepaths(dir: &str) -> Vec<PathBuf> {
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
pub fn cp(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_command("cp", &["-r", from, to])
}
pub fn write_file(path: impl AsRef<Path>, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
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
            "target/debug/cw"
        );
    }
}
