use clap::{Parser, Subcommand};
use cw::{
    build::{BuildMode, CargoBuildWrapper},
    new::CargoNewWrapper,
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
        Sub::New { cli, name, web } => {
            let new = CargoNewWrapper::new(name);
            if cli {
                return new.create_new_cli_project();
            }
            if web {
                return new.create_new_web_project();
            }
            new.create_new_project()
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
    },
}
