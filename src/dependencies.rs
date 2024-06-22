use serde::{Deserialize, Serialize};

fn construct_pypi_url(package_name: &str) -> String {
    format!("https://pypi.org/pypi/{package_name}/json")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectDependency {
    pub name: String,
    pub version: String,
    pub extras: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PyPIDependency {
    info: PyPIDependencyInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PyPIDependencyInfo {
    version: String,
}

impl ProjectDependency {
    pub fn new(raw_dependency_string: &str) -> Self {
        let mut parts = raw_dependency_string.splitn(2, "==");

        let mut name = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        let extras = if name.contains('[') && name.contains(']') {
            let start = name.find('[').unwrap();
            let end = name.find(']').unwrap();

            let extras_vec = name[start..=end]
                .split(',')
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();

            name = name[..start].to_string();

            extras_vec
        } else {
            vec![]
        };

        Self {
            name,
            version,
            extras,
        }
    }

    pub fn fetch_current_pypi_version(&self) -> String {
        let url = construct_pypi_url(self.name.as_str());

        let response = reqwest::blocking::get(url).unwrap();

        let pypi_dependency_data: PyPIDependency = response.json().unwrap();

        pypi_dependency_data.info.version
    }
}
