use std::fs;

pub struct TemplateConfig {
    pub validation_regexp: String,
    pub destination: String,
}

impl TemplateConfig {
    pub fn new(template_path: &str) -> () {
        let template_file = fs::read_to_string(template_path).unwrap();

        
    }
}
