use std::env;

const ALL_ARG: &str = "-a";

pub struct CLIConfig {
    pub component_name: String,
    pub create_all: bool,
}

impl CLIConfig {
    pub fn new(mut args: env::Args) -> Result<CLIConfig, &'static str> {
        // skip binary path
        args.next();

        let mut are_all_applied = false;
        let mut component_name: Option<String> = Option::None;

        loop {
            if let Some(arg) = args.next() {
                if arg == ALL_ARG {
                    are_all_applied = true;
                } else if component_name.is_none() {
                    if is_valid_name(&arg) {
                        component_name = Some(arg);
                    } else {
                        return Result::Err(
                            "Invalid component name, it should start with an uppercase letter.",
                        );
                    }
                } else {
                    return Result::Err(
                      "Invalid arguments. The CLI accepts only the component name and the '-a' flag.",
                  );
                }
            } else if component_name.is_none() {
                return Result::Err("Component name not provided!");
            } else {
                break;
            }
        }

        Result::Ok(CLIConfig {
            create_all: are_all_applied,
            component_name: component_name.unwrap(),
        })
    }
}

fn is_valid_name(name: &str) -> bool {
    return name.chars().next().unwrap().is_ascii_uppercase();
}
