use color_eyre::eyre::{eyre, Context, Result};
use regex::{Match, Regex};

#[derive(Debug, Clone)]
pub struct SectionLocation {
    index: i32,
    length: Option<i32>,
}

impl SectionLocation {
    // pub fn new(raw_config: &str, constraint: Option<SearchConstraint>) -> Self {
    pub fn new(raw_section_matches: &[Match], query: &str) -> Option<Self> {
        for section_match in raw_section_matches {
            if section_match.as_str() == query {
                return Some(Self {
                    index: todo!(),
                    length: todo!(),
                });
            }
        }
        // for (i, line) in config_vec.iter().enumerate() {
        //     let re = Regex::new(format!(r"^\[{query}\]$").as_str()).unwrap();
        // }

        None
    }
}
