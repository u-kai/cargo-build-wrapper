use std::path::Path;

use crate::{
    cargo_toml::CargoTomlContent,
    commands::{run_command, write_file},
};

pub struct CargoNewWrapper {
    name: String,
}
impl CargoNewWrapper {
    const CLAP_VERSION: &'static str = "4.2.1";
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    pub fn create_new_cli_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_new_project()?;
        let project_root: &Path = self.name.as_ref();
        let mut cargo_toml_content = CargoTomlContent::new(self.name.as_str());
        cargo_toml_content.add_depend("clap", Self::CLAP_VERSION, ("features", vec!["derive"]));
        let cargo_toml_content = cargo_toml_content.gen();
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
    pub fn create_new_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        run_command("cargo", &["new", self.name.as_str()])
    }
}
