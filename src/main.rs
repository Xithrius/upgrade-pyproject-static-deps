#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::future_not_send)]

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{eyre, Result};

use projects::ProjectConfig;

mod dependencies;
mod projects;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // What pyproject file this project should operate off of
    pyproject: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let cli = Cli::parse();

    let pyproject_path = cli.pyproject;

    if !pyproject_path.exists() || pyproject_path.file_name().unwrap() != "pyproject.toml" {
        return Err(eyre!(
            "`pyproject.toml` does not exist at the path {:?}",
            pyproject_path.display()
        ));
    }

    let pyproject_config = ProjectConfig::new(pyproject_path);

    let deps = pyproject_config.get_dependencies();

    for dep in deps {
        let upstream_version = dep.fetch_current_pypi_version();
        println!(
            "Dependency {:?} is version {:?}, upstream is {:?}",
            dep.name, dep.version, upstream_version,
        );
    }

    Ok(())
}
