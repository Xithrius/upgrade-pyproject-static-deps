#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::future_not_send)]

use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::{eyre, Context, Result};

use dependencies::ProjectDependency;
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

    if !(pyproject_path.exists()
        && pyproject_path.is_file()
        && pyproject_path.file_name().unwrap() == "pyproject.toml")
    {
        return Err(eyre!(
            "`pyproject.toml` does not exist at the path {:?}",
            pyproject_path.display()
        ));
    }

    let raw_pyproject_toml_str = fs::read_to_string(pyproject_path.clone())
        .wrap_err("Failed to read file to string.")
        .unwrap();

    let mut pyproject_config = ProjectConfig::new(raw_pyproject_toml_str.as_str());

    let deps: Vec<ProjectDependency> = pyproject_config
        .get_dependencies()
        .iter_mut()
        .map(|d| {
            let upstream_version = d.fetch_current_pypi_version();

            if upstream_version > d.version {
                println!("{} {} -> {}", d.raw_dependency, d.version, upstream_version);
                d.version = upstream_version;
            }

            d.clone()
        })
        .collect();

    let serialized_deps = deps
        .iter()
        .map(ProjectDependency::to_string)
        .collect::<Vec<String>>();

    pyproject_config.set_dependencies_vec(serialized_deps);

    let serialized_config = toml::to_string(&pyproject_config).unwrap();

    let _ = fs::write(pyproject_path, serialized_config.as_bytes());

    Ok(())
}
