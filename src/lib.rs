pub mod config;

use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use dialoguer::MultiSelect;

const TEMPLATE_FOLDER: &str = "cg_template";

pub fn run(config: Config) -> Result<(), String> {
    println!("Config: {:#?}", config);

    let dir_path = Path::new(TEMPLATE_FOLDER);

    if !dir_path.is_dir() {
        return Result::Err(format!(
            "Template folder not found. Please create \"{}\"",
            TEMPLATE_FOLDER,
        ));
    }

    let mut templates = get_dir_files(dir_path)?;
    templates = if config.generate_all {
        templates
    } else {
        let items_to_select = templates
            .iter()
            .map(|template| template.to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        let selected_items: Vec<&str> = select(&items_to_select)?;

        templates
            .into_iter()
            .filter(|template| selected_items.contains(&template.to_str().unwrap()))
            .collect::<Vec<PathBuf>>()
    };

    println!("Templates: {:#?}", templates);

    return Result::Ok(());
}

fn get_dir_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut templates = Vec::new();

    for template in fs::read_dir(path).map_err(|err| err.to_string())? {
        let template = template.map_err(|err| err.to_string())?;
        let path = template.path();

        if path.is_file() {
            templates.push(path);
        }
    }

    if templates.len() != 0 {
        return Result::Ok(templates);
    } else {
        return Result::Err(String::from("Template files not found."));
    }
}

fn select<'a>(items: &'a Vec<String>) -> Result<Vec<&'a str>, String> {
    let selected_items_indexes = MultiSelect::new()
        .items(&items)
        .interact()
        .map_err(|err| err.to_string())?;

    let selected_items = items
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| {
            if selected_items_indexes.contains(&index) {
                return Some(item.as_str());
            }

            return None;
        })
        .collect();

    return Result::Ok(selected_items);
}
