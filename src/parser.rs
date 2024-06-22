use std::{fs, path::PathBuf};

use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub project: ProjectAttributes,

    #[serde(flatten)]
    other: toml::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectAttributes {
    pub dependencies: Vec<String>,

    #[serde(flatten)]
    other: toml::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectDependency {
    name: String,
    version: String,
}

impl ProjectDependency {
    pub fn new(raw_dependency_string: &str) -> Self {
        let mut parts = raw_dependency_string.splitn(2, "==");

        let name = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        Self { name, version }
    }
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

    fn get_raw_dependencies(&self) -> Vec<String> {
        self.project.dependencies.clone()
    }

    fn get_dependencies(&self) -> Vec<ProjectDependency> {
        self.project
            .dependencies
            .clone()
            .iter()
            .map(|v| ProjectDependency::new(v))
            .collect()
    }
}

fn fetch_pypi_dependency_data() {
    todo!()
}
