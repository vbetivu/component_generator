pub mod config;
pub mod constants;
pub mod utils;

use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use constants::{COMPONENT_REPLACE_PATTERN, TEMPLATE_FOLDER, TRANSFORM_KEBAB_CASE};
use utils::{get_dir_files, pascal_to_kebab, select};

struct Template {
    config: Config,
    templates: Vec<PathBuf>,
}

impl Template {
    fn new(config: Config, templates: Vec<PathBuf>) -> Template {
        Template { config, templates }
    }

    fn generate(&self) -> Result<(), String> {
        let destination_dir = Path::new(&self.config.dir);

        if !destination_dir.is_dir() {
            return Result::Err(String::from("Destination dir not found."));
        }

        let component_dir = destination_dir.join(&self.config.component_name);

        fs::create_dir(&component_dir).map_err(|err| -> String { err.to_string() })?;

        for template in &self.templates {
            self.create_file(template)?;
        }

        println!("{}", component_dir.to_str().unwrap());

        return Result::Ok(());
    }

    fn create_file(&self, template: &PathBuf) -> Result<(), String> {
        let mut contents = fs::read_to_string(template).unwrap();
        let destination = Path::new(&self.config.dir)
            .join(&self.config.component_name)
            .join(
                template
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(COMPONENT_REPLACE_PATTERN, &self.config.component_name),
            );

        contents = contents.replace(COMPONENT_REPLACE_PATTERN, &self.config.component_name);

        for (arg, value) in &self.config.extra_args {
            match value.as_str() {
                TRANSFORM_KEBAB_CASE => {
                    contents = contents.replace(
                        &format!("{{{{{}}}}}", arg),
                        &pascal_to_kebab(&self.config.component_name),
                    )
                }
                _ => contents = contents.replace(&format!("{{{{{}}}}}", arg), value),
            }
        }

        fs::write(destination, contents).map_err(|err| -> String { err.to_string() })?;

        Result::Ok(())
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let dir_path = Path::new(TEMPLATE_FOLDER);

    if !dir_path.is_dir() {
        return Result::Err(format!(
            "Template folder is missing. Please create \"{}\".",
            TEMPLATE_FOLDER
        ));
    }

    let templates = get_dir_files(dir_path).map_err(|err| -> String { err.to_string() })?;
    let selected_templates = if config.generate_all {
        templates
    } else {
        select(templates)?
    };

    let template = Template::new(config, selected_templates);

    template.generate()?;

    return Result::Ok(());
}
