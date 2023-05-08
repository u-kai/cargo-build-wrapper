use std::path::Path;

use crate::{
    cargo_toml::{self, CargoTomlContent},
    commands::{run_command, write_file},
    new::struct_builder::StructBuilder,
};

use super::code_builder::MainRsBuilder;

#[derive(Debug)]
pub struct RustNewProjectOptions {
    name: String,
    cli: bool,
    web: bool,
    remote_client: bool,
    docker_file: bool,
}

impl RustNewProjectOptions {}

#[derive(Debug)]
pub struct CargoProjectCreator {
    name: String,
    cargo_toml_content: CargoTomlContent,
    main_rs: MainRsBuilder,
}

impl CargoProjectCreator {
    const CLAP_VERSION: &'static str = "4.2.1";
    const ACTIX_WEB_VERSION: &'static str = "4";
    const TOKIO_VERSION: &'static str = "1";
    const REQWEST_VERSION: &'static str = "0.11";
    const SERDE_VERSION: &'static str = "1";
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            cargo_toml_content: CargoTomlContent::new(name.as_str()),
            name,
            main_rs: MainRsBuilder::new(),
        }
    }
    pub fn cli(mut self) -> Self {
        self.cargo_toml_content.add_depend_with_attr(
            "clap",
            Self::CLAP_VERSION,
            ("features", vec!["derive"]),
        );
        let struct_builder = StructBuilder::new("Cli")
            .add_derive("Parser")
            .add_field_with_attr("sub", "Sub", "clap(subcommand)");
        let enum_builder = StructBuilder::new_enum("Sub")
            .add_derive("Subcommand")
            .add_inner_comment("sub command hear")
            .add_inner_comment("#[clap(short, long)]");
        self.main_rs = self
            .main_rs
            .add_depend("use clap::{Parser, Subcommand};")
            .add_main_line("let cli = Cli::parse();")
            .add_struct_builder(struct_builder)
            .add_struct_builder(enum_builder);
        self
    }
    pub fn remote_client(mut self) -> Self {
        self.cargo_toml_content.add_depend_with_attr(
            "tokio",
            Self::TOKIO_VERSION,
            ("features", vec!["full"]),
        );
        self.cargo_toml_content
            .add_depend("reqwest", Self::REQWEST_VERSION);
        self.cargo_toml_content.add_depend_with_attr(
            "serde",
            Self::SERDE_VERSION,
            ("features", vec!["derive"]),
        );
        self.cargo_toml_content
            .add_depend("serde_json", Self::SERDE_VERSION);
        self.main_rs = self.main_rs
            .add_depend("use reqwest::Client;")
            .add_depend("use std::collections::HashMap;")
            .add_main_line("let mut map = HashMap::new();")
            .add_main_line("map.insert(\"lang\",\"rust\");")
            .add_main_line("map.insert(\"body\",\"json\");")
            .add_main_line("let client = Client::new();")
            .add_main_line("let res = client.post(\"http://httpbin.org/post\").json(&map).send().await.unwrap();")
            .async_main_mode()
            .main_attr("tokio::main");
        self
    }
    pub fn create_new_project(self) -> Result<(), Box<dyn std::error::Error>> {
        let project_root: &Path = self.name.as_ref();
        let cargo_toml_content = self.cargo_toml_content.gen();
        let main_rs = self.main_rs.build();
        run_command("cargo", &["new", self.name.as_str()])?;
        write_file(project_root.join("Cargo.toml"), &cargo_toml_content)?;
        write_file(project_root.join("src/main.rs"), &main_rs)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn remote_client_snap_shot_test() {
        let remote_client = CargoProjectCreator::new("remote_client")
            .remote_client()
            .main_rs
            .build();
        assert_eq!(
            remote_client,
            r#"use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert("lang","rust");
    map.insert("body","json");
    let client = Client::new();
    let res = client.post("http://httpbin.org/post").json(&map).send().await.unwrap();
}"#
        );
    }
    #[test]
    fn cli_snap_shot_test() {
        let cli = CargoProjectCreator::new("cli").cli().main_rs.build();
        assert_eq!(
            cli,
            r#"use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    sub: Sub,
}
#[derive(Subcommand)]
enum Sub {
    // sub command hear
    // #[clap(short, long)]
}"#
        )
    }
}

pub struct CargoNewWrapper {
    name: String,
}
impl CargoNewWrapper {
    const CLAP_VERSION: &'static str = "4.2.1";
    const ACTIX_WEB_VERSION: &'static str = "4";
    const TOKIO_VERSION: &'static str = "1";
    const REQWEST_VERSION: &'static str = "0.11";
    const SERDE_VERSION: &'static str = "1";
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    pub fn create_new_client_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_new_project()?;
        let project_root: &Path = self.name.as_ref();
        let mut cargo_toml_content = CargoTomlContent::new(self.name.as_str());
        cargo_toml_content.add_depend_with_attr(
            "tokio",
            Self::TOKIO_VERSION,
            ("features", vec!["full"]),
        );
        cargo_toml_content.add_depend("reqwest", Self::REQWEST_VERSION);
        cargo_toml_content.add_depend_with_attr(
            "serde",
            Self::SERDE_VERSION,
            ("features", vec!["derive"]),
        );
        cargo_toml_content.add_depend("serde_json", Self::SERDE_VERSION);
        let main = format!(
            r#"#[tokio::main]
async fn main() {{
    let mut map = std::collections::HashMap::new();
    map.insert("lang","rust");
    map.insert("body","json");
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post").json(&map).send().await().unwrap();
}}"#
        );
        write_file(project_root.join("Cargo.toml"), &cargo_toml_content.gen())?;
        write_file(project_root.join("src/main.rs"), &main)?;
        Ok(())
    }
    pub fn create_new_web_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_new_project()?;
        let project_root: &Path = self.name.as_ref();
        let mut cargo_toml_content = CargoTomlContent::new(self.name.as_str());
        cargo_toml_content.add_depend_with_attr(
            "actix-web",
            Self::ACTIX_WEB_VERSION,
            ("features", vec!["rustls"]),
        );
        cargo_toml_content.add_depend("rustls", "0.21.0");
        let main = format!(
            r#"use actix_web::{{web, App, HttpResponse, HttpServer, Responder}};
            
#[actix_web::get("/")]
async fn hello() -> impl Responder {{
    HttpResponse::Ok().body("Hello world!")
}}
            
#[actix_web::post("/echo")]
async fn echo(req_body: String) -> impl Responder {{
    HttpResponse::Ok().body(req_body)
}}

#[actix_web::main]
async fn main() -> std::io::Result<()> {{
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}}"#
        );
        write_file(project_root.join("Cargo.toml"), &cargo_toml_content.gen())?;
        write_file(project_root.join("src/main.rs"), &main)?;
        write_file(project_root.join("Dockerfile"), &self.create_docker_file())?;
        Ok(())
    }
    pub fn create_new_cli_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_new_project()?;
        let project_root: &Path = self.name.as_ref();
        let mut cargo_toml_content = CargoTomlContent::new(self.name.as_str());
        cargo_toml_content.add_depend_with_attr(
            "clap",
            Self::CLAP_VERSION,
            ("features", vec!["derive"]),
        );
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

    fn create_docker_file(&self) -> String {
        format!(
            r#"FROM ekidd/rust-musl-builder:1.51.0 AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release
        
# final. application layer
FROM busybox:musl
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/{} ./{}
CMD ["./{}"]"#,
            self.name.as_str(),
            self.name.as_str(),
            self.name.as_str()
        )
    }
}

//#[cfg(test)]
//mod tests {
//#[test]
//fn cli_snap_shot_test(){
//let
//}
//}
