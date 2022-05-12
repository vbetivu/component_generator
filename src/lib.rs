pub mod config;

use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use dialoguer::MultiSelect;

const TEMPLATE_FOLDER: &str = "cg_template";
const COMPONENT_REPLACE_PATTERN: &str = "{{component}}";

struct Template {
    config: Config,
    templates: Vec<PathBuf>,
}

impl Template {
    fn generate(&self) -> Result<(), String> {
        let destination_dir = Path::new(&self.config.dir);

        if !destination_dir.is_dir() {
            return Result::Err(String::from("Destination dir not found."));
        }

        let destination_dir = destination_dir.join(&self.config.component_name);

        fs::create_dir(&destination_dir).map_err(|err| err.to_string())?;

        println!("Created: {}", destination_dir.to_str().unwrap());

        for template in &self.templates {
            self.create_template_file(&template, &destination_dir)?;
        }

        Result::Ok(())
    }

    fn create_template_file(
        &self,
        template: &PathBuf,
        destination_dir: &PathBuf,
    ) -> Result<(), String> {
        let mut contents = fs::read_to_string(template).unwrap();
        let destination = destination_dir.join(
            template
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(COMPONENT_REPLACE_PATTERN, &self.config.component_name),
        );

        contents = contents.replace(COMPONENT_REPLACE_PATTERN, &self.config.component_name);

        for (arg, value) in &self.config.extra_args {
            contents = contents.replace(&format!("{{{{{}}}}}", arg), value);
        }

        fs::write(&destination, contents).map_err(|err| err.to_string())?;

        println!("Created: {}", destination.to_str().unwrap());

        Result::Ok(())
    }
}

pub fn run(config: Config) -> Result<(), String> {
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

    let template = Template { config, templates };

    template.generate()?;

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
