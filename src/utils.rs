use dialoguer::MultiSelect;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::constants::PASCAL_CASE_REGEXP;

pub fn get_dir_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut templates: Vec<PathBuf> = Vec::new();

    for template in fs::read_dir(path).map_err(|err| err.to_string())? {
        let template = template.map_err(|err| err.to_string())?;
        let path = template.path();

        if path.is_file() {
            templates.push(path);
        }
    }

    if templates.len() != 0 {
        Result::Ok(templates)
    } else {
        Result::Err(String::from("Template files not found."))
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

pub fn select<'a>(items: &'a Vec<String>) -> Result<Vec<&'a str>, String> {
    let selected_items_indexes = MultiSelect::new()
        .items(&items)
        .interact()
        .map_err(|err| -> String { err.to_string() })?;

    let selected_items = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if selected_items_indexes.contains(&i) {
                Some(item.as_str())
            } else {
                None
            }
        })
        .collect();

    Result::Ok(selected_items)
}
