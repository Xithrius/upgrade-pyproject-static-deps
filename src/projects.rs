#[allow(unused_imports)]
use color_eyre::eyre::{eyre, Context, Result};
use once_cell::sync::Lazy;
use regex::{Match, Regex};

use crate::dependencies::locations::DependencyLocation;

pub static SECTION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[[\w\.-]+\]$").unwrap());

#[derive(Debug, Clone)]
pub struct ProjectConfigParser {
    dependency_locations: Vec<DependencyLocation>,
}

#[allow(dead_code, unused_variables)]
impl ProjectConfigParser {
    pub fn new(raw_config: &str) -> Result<Self> {
        let config_vec = raw_config
            .clone()
            .split('\n')
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        let raw_section_matches = SECTION_REGEX.find_iter(raw_config).collect::<Vec<Match>>();

        todo!()
    }

    // pub fn set_dependencies_vec(&mut self, new_dependencies: Vec<String>) {
    //     self.project.dependencies = new_dependencies;
    // }

    // #[allow(dead_code)]
    // pub fn get_raw_dependencies(&self) -> Vec<String> {
    //     self.project.dependencies.clone()
    // }

    // pub fn get_dependencies(&self) -> Vec<ProjectDependency> {
    //     self.project
    //         .dependencies
    //         .clone()
    //         .iter()
    //         .map(|v| ProjectDependency::new(v))
    //         .collect()
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic = "missing field `dependencies`"]
//     fn test_parsing_invalid_pyproject_with_no_dependencies_attribute() {
//         let raw_config_str: &str = "[project]";
//         ProjectConfig::new(raw_config_str);
//     }

//     #[test]
//     fn test_parsing_empty_dependencies_returns_empty_vector() {
//         let raw_config_str: &str = "[project]\ndependencies=[]";
//         let config = ProjectConfig::new(raw_config_str);

//         assert!(config.get_dependencies().is_empty());
//     }
// }
