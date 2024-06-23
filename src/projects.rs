use color_eyre::eyre::Context;
use serde::{Deserialize, Serialize};

use crate::dependencies::ProjectDependency;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectConfig {
    project: ProjectAttributes,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectAttributes {
    dependencies: Vec<String>,
}

impl ProjectConfig {
    pub fn new(raw_config: &str) -> Self {
        let config: Self = toml::from_str(raw_config)
            .wrap_err("Failed to deserialize `pyproject.toml` file.")
            .unwrap();

        config
    }

    pub fn set_dependencies_vec(&mut self, new_dependencies: Vec<String>) {
        self.project.dependencies = new_dependencies;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "missing field `dependencies`"]
    fn test_parsing_invalid_pyproject_with_no_dependencies_attribute() {
        let raw_config_str: &str = "[project]";
        ProjectConfig::new(raw_config_str);
    }

    #[test]
    fn test_parsing_empty_dependencies_returns_empty_vector() {
        let raw_config_str: &str = "[project]\ndependencies=[]";
        let config = ProjectConfig::new(raw_config_str);

        assert!(config.get_dependencies().is_empty());
    }
}
