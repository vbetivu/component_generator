pub mod config;

use config::Config;

struct Template {
    config: Config,
}

impl Template {
    fn new(config: Config) -> Template {
        Template { config }
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        println!(
            "{} {} {}",
            self.config.dir, self.config.generate_all, self.config.component_name
        );

        return Result::Ok(());
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let mut text = Template::new(config);

    text.generate()?;

    return Result::Ok(());
}
