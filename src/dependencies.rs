use std::{fmt::Display, string::ToString};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectDependency {
    pub name: String,
    pub version: String,
    pub extras: Option<ProjectDependencyExtras>,
    pub raw_dependency: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectDependencyExtras {
    pub features: Vec<String>,
    pub raw_string: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PyPIDependency {
    info: PyPIDependencyInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PyPIDependencyInfo {
    version: String,
}

impl ProjectDependency {
    pub fn new(raw_dependency_string: &str) -> Self {
        let mut parts = raw_dependency_string.splitn(2, "==");

        let mut name = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        let raw_dependency = name.clone();

        let extras = if name.contains('[') && name.contains(']') {
            let start = name.find('[').unwrap();
            let end = name.find(']').unwrap();

            let raw_extras = name[start..=end].to_string();
            let features = raw_extras
                .split(',')
                .map(ToString::to_string)
                .collect::<Vec<String>>();

            name = name[..start].to_string();

            Some(ProjectDependencyExtras::new(features, raw_extras))
        } else {
            None
        };

        Self {
            name,
            version,
            extras,
            raw_dependency,
        }
    }

    fn construct_pypi_url(package_name: &str) -> String {
        format!("https://pypi.org/pypi/{package_name}/json")
    }

    pub fn fetch_current_pypi_version(&self) -> String {
        let url = Self::construct_pypi_url(self.name.as_str());

        let response = reqwest::blocking::get(url).unwrap();

        let pypi_dependency_data: PyPIDependency = response.json().unwrap();

        pypi_dependency_data.info.version
    }
}

impl Display for ProjectDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.extras.as_ref().map_or_else(
                || format!("{}=={}", self.name, self.version),
                |extras| format!("{}{}=={}", self.name, extras.raw_string, self.version)
            )
        )
    }
}

impl ProjectDependencyExtras {
    pub fn new(features: Vec<String>, raw_string: String) -> Self {
        Self {
            features,
            raw_string,
        }
    }
}
