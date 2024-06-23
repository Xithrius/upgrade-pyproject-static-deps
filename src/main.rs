#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::future_not_send)]

mod dependencies;
mod files;
mod parser;
mod projects;
mod sections;

use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::{eyre, Context, Result};
use inquire::Confirm;

use files::{get_modification_time, has_file_been_modified};
use projects::ProjectConfigParser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What pyproject file this application should operate off of
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

    let last_checked_at = get_modification_time(pyproject_path.clone());

    let raw_pyproject_toml_str = fs::read_to_string(pyproject_path.clone())
        .wrap_err("Failed to read file to string.")
        .unwrap();

    #[allow(unused_variables)]
    let parsed_project_config = ProjectConfigParser::new(raw_pyproject_toml_str.as_str())
        .wrap_err("Failed to parse `pyproject.toml`")
        .unwrap();

    // This must go at the end of the function before writing
    if let Some(pyproject_checked_at) = last_checked_at {
        if has_file_been_modified(pyproject_path, pyproject_checked_at) {
            // File has been modified since the time it was read into memory,
            // user must choose if the current file should be overwritten.
            let ans = Confirm::new("File has been modified since program started. Overwrite?")
            .with_default(false)
            .with_help_message(
                "File metadata shows modification datetime comes after program startup datetime.",
            )
            .prompt()
            .expect("Something went wrong with the confirmation prompt.");

            if !ans {
                return Ok(());
            }
        }
    }

    Ok(())
}
