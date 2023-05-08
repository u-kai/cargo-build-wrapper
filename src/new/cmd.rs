use std::path::Path;

use crate::{
    cargo_toml::CargoTomlContent,
    commands::{run_command, write_file},
    new::struct_builder::StructBuilder,
};

use super::{code_builder::MainRsBuilder, fn_builder::FnBuilder};

#[derive(Debug)]
pub struct RustNewProjectOptions {
    pub name: String,
    pub cli: bool,
    pub web: bool,
    pub remote_client: bool,
    pub docker_file: bool,
}
#[derive(Debug)]
pub struct CargoProjectCreator {
    name: String,
    cargo_toml_content: CargoTomlContent,
    depend_store: Vec<String>,
    main_rs: MainRsBuilder,
}

impl CargoProjectCreator {
    const CLAP_VERSION: &'static str = "4.2.1";
    const ACTIX_WEB_VERSION: &'static str = "4.3.1";
    const TOKIO_VERSION: &'static str = "1";
    const REQWEST_VERSION: &'static str = "0.11";
    const SERDE_VERSION: &'static str = "1";
    pub fn create_project_from_options(
        options: RustNewProjectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut creator = Self::new(options.name);
        if options.cli {
            creator = creator.cli();
        }
        if options.remote_client {
            creator = creator.remote_client();
        }
        if options.web {
            creator = creator.web_server();
        }
        if options.docker_file {
            creator.create_new_project_with_docker()
        } else {
            creator.create_new_project()
        }
    }
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            depend_store: Vec::new(),
            cargo_toml_content: CargoTomlContent::new(name.as_str()),
            name,
            main_rs: MainRsBuilder::new(),
        }
    }
    pub fn web_server(mut self) -> Self {
        self.add_actix_web();
        let fn_builder = FnBuilder::new("index")
            .retu("impl Responder")
            .async_mode()
            .add_line("\"Hello world!\"")
            .attr(r#"actix_web::get("/")"#);
        let fn_builder_query = FnBuilder::new("with_query")
            .retu("impl Responder")
            .async_mode()
            .attr(r#"actix_web::get("/with_query")"#)
            .add_arg("info", "web::Query<Info>")
            .add_line("let response = format!(\"Hello, {}! You are {} years old.\", info.name, info.age);")
            .add_line("HttpResponse::Ok().body(response)");
        let struct_builder = StructBuilder::new("Info")
            .add_derive("Deserialize")
            .add_field_with_attr("name", "String", r#"serde(rename = "name")"#)
            .add_field_with_attr("age", "u8", r#"serde(rename = "age")"#);
        self.main_rs = self
            .main_rs
            .add_fn_builder(fn_builder)
            .add_fn_builder(fn_builder_query)
            .add_struct_builder(struct_builder)
            .add_depend("use actix_web::{web,App, HttpResponse, HttpServer, Responder};")
            .add_depend("use serde::Deserialize;")
            .async_main_mode()
            .main_return("std::io::Result<()>")
            .main_attr("actix_web::main")
            .add_main_line(r#"HttpServer::new(|| App::new().service(index).service(with_query)).bind(("127.0.0.1", 8080))?;"#)
            .add_main_line("Ok(())");
        self
    }
    pub fn cli(mut self) -> Self {
        self.add_clap();
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
        self.add_tokio();
        self.add_reqwest();
        self.add_serde();
        self.main_rs = self
            .main_rs
            .add_depend("use reqwest::Client;")
            .add_main_line("let client = Client::new();")
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
    pub fn create_new_project_with_docker(self) -> Result<(), Box<dyn std::error::Error>> {
        let name = self.name.clone();
        self.create_new_project()?;
        Self::docker_file(name.as_str())
    }
    pub fn add_tokio(&mut self) {
        if self.depend_store.contains(&"tokio".to_string()) {
            return;
        }
        self.depend_store.push("tokio".to_string());
        self.cargo_toml_content.add_depend_with_attr(
            "tokio",
            Self::TOKIO_VERSION,
            ("features", vec!["full"]),
        );
    }
    pub fn add_clap(&mut self) {
        if self.depend_store.contains(&"clap".to_string()) {
            return;
        }
        self.depend_store.push("clap".to_string());
        self.cargo_toml_content.add_depend_with_attr(
            "clap",
            Self::CLAP_VERSION,
            ("features", vec!["derive"]),
        );
    }
    pub fn add_reqwest(&mut self) {
        if self.depend_store.contains(&"reqwest".to_string()) {
            return;
        }
        self.depend_store.push("reqwest".to_string());
        self.cargo_toml_content
            .add_depend("reqwest", Self::REQWEST_VERSION);
    }
    pub fn add_actix_web(&mut self) {
        if self.depend_store.contains(&"actix-web".to_string()) {
            return;
        }
        self.depend_store.push("actix-web".to_string());
        self.cargo_toml_content.add_depend_with_attr(
            "actix-web",
            Self::ACTIX_WEB_VERSION,
            ("features", vec!["rustls"]),
        );
    }
    pub fn add_serde(&mut self) {
        if self.depend_store.contains(&"serde".to_string()) {
            return;
        }
        self.depend_store.push("serde".to_string());
        self.cargo_toml_content.add_depend_with_attr(
            "serde",
            Self::SERDE_VERSION,
            ("features", vec!["derive"]),
        );
        self.cargo_toml_content
            .add_depend("serde_json", Self::SERDE_VERSION);
    }
    fn docker_file(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = format!(
            r#"FROM ekidd/rust-musl-builder:1.51.0 AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release
        
# final. application layer
FROM busybox:musl
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/{} ./{}
CMD ["./{}"]"#,
            name, name, name
        );
        let path: &Path = name.as_ref();
        write_file(path.join("Dockerfile"), &content)?;
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

#[tokio::main]
async fn main() {
    let client = Client::new();
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
