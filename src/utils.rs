use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::constants::PASCAL_CASE_REGEXP;

pub fn get_dir_files(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut templates: Vec<PathBuf> = Vec::new();

    for template_entry in fs::read_dir(path)? {
        let template_entry = template_entry?;
        let path = template_entry.path();

        if !path.is_dir() {
            templates.push(path);
        }
    }

    if templates.len() != 0 {
        Result::Ok(templates)
    } else {
        Result::Err(io::Error::new(io::ErrorKind::Other, "Empty directory."))
    }
}

pub fn pascal_to_kebab(text: &str) -> String {
    Regex::new(PASCAL_CASE_REGEXP)
        .unwrap()
        .find_iter(text)
        .map(|x| -> String { return x.as_str().to_ascii_lowercase() })
        .collect::<Vec<String>>()
        .join("-")
}
