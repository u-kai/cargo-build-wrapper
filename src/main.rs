use clap::{Parser, Subcommand};
use cw::{
    build::{BuildMode, CargoBuildWrapper},
    new::cmd::{CargoProjectCreator, RustNewProjectOptions},
};

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
        Sub::New {
            cli,
            name,
            web,
            remote_client,
            docker_file,
        } => {
            let options = RustNewProjectOptions {
                name,
                cli,
                web,
                remote_client,
                docker_file,
            };
            CargoProjectCreator::create_project_from_options(options)
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
        #[clap(short, long)]
        web: bool,
        #[clap(short, long)]
        remote_client: bool,
        #[clap(short, long)]
        docker_file: bool,
    },
}
