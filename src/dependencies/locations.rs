use std::{fmt::Display, string::ToString};

// fn construct_pypi_url(package_name: &str) -> String {
//     format!("https://pypi.org/pypi/{package_name}/json")
// }

// pub fn fetch_current_pypi_version(&self) -> String {
//     let url = Self::construct_pypi_url(self.name.as_str());

//     let response = reqwest::blocking::get(url).unwrap();

//     let pypi_dependency_data: PyPIDependency = response.json().unwrap();

//     pypi_dependency_data.info.version
// }

#[derive(Debug, Clone)]
pub struct DependencyLocation {
    start: i32,
    end: i32,
    length: i32,
}
