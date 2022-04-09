pub mod configs;

use configs::{
    cli_config::CLIConfig, constants::TEMPLATE_FILE, constants::TEMPLATE_FOLDER,
    template_config::TemplateConfig,
};

struct Template {
    cli_config: CLIConfig,
    template_config: TemplateConfig,
}

impl Template {
    fn new(cli_config: CLIConfig) -> Template {
        let template_config =
            TemplateConfig::new(&format!("./{}/{}", TEMPLATE_FOLDER, TEMPLATE_FILE));

        Template {
            cli_config,
            template_config,
        }
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        println!(
            "{} {}",
            self.cli_config.create_all, self.cli_config.component_name
        );

        println!("{:?}", self.template_config);

        return Result::Ok(());
    }
}

pub fn run(config: CLIConfig) -> Result<(), &'static str> {
    let mut text = Template::new(config);

    text.generate()?;

    return Result::Ok(());
}
