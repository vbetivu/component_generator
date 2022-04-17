pub mod config;
pub mod constants;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use config::Config;
use constants::TEMPLATE_FOLDER;

struct Template {
    config: Config,
}

impl Template {
    fn new(config: Config) -> Template {
        Template { config }
    }

    fn generate(&mut self) -> Result<(), String> {
        println!("{:#?}", self.config);

        let dir_path = Path::new(TEMPLATE_FOLDER);

        if !dir_path.is_dir() {
            return Result::Err(format!(
                "Template folder is missing. Please create \"{}\".",
                TEMPLATE_FOLDER
            ));
        }

        let templates = read_templates(dir_path).unwrap();
        println!("{:#?}", templates);

        return Result::Ok(());
    }
}

fn read_templates(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
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

pub fn run(config: Config) -> Result<(), String> {
    let mut text = Template::new(config);

    text.generate()?;

    return Result::Ok(());
}
