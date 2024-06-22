use std::{fs, path::PathBuf};

use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};

use crate::dependencies::ProjectDependency;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    project: ProjectAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectAttributes {
    dependencies: Vec<String>,
}

impl ProjectConfig {
    pub fn new(path: PathBuf) -> Self {
        let toml_data = fs::read_to_string(path)
            .wrap_err("Failed to read file to string.")
            .unwrap();
        let config: Self = toml::from_str(&toml_data)
            .wrap_err("Failed to deserialize `pyproject.toml` file.")
            .unwrap();

        config
    }

    #[allow(dead_code)]
    pub fn get_raw_dependencies(&self) -> Vec<String> {
        self.project.dependencies.clone()
    }

    pub fn get_dependencies(&self) -> Vec<ProjectDependency> {
        self.project
            .dependencies
            .clone()
            .iter()
            .map(|v| ProjectDependency::new(v))
            .collect()
    }
}
