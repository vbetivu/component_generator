pub mod config;

use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;

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

    let templates = get_dir_files(dir_path)?;

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
