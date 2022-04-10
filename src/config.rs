use std::env;

use regex::Regex;

const ALL_ARG: &str = "-a";
const DIR_ARG: &str = "-d";
const REGEX_ARG: &str = "-r";

pub struct Config {
    pub component_name: String,
    pub generate_all: bool,
    pub dir: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip binary path
        args.next();

        let mut component_name = Option::None;
        let mut generate_all = false;
        let mut dir = Option::None;
        let mut name_validator = Option::None;

        loop {
            if let Some(arg) = args.next() {
                let arg_str = arg.as_str();
                match arg_str {
                    ALL_ARG => generate_all = true,
                    DIR_ARG => dir = Some(args.next().expect("Directory not provided.")),
                    REGEX_ARG => {
                        name_validator = Some(args.next().expect("Validator not provided."))
                    }

                    _ => {
                        if component_name.is_none() {
                            component_name = Some(arg)
                        } else {
                            return Result::Err("Invalid arguments.");
                        }
                    }
                }
            } else if component_name.is_none() {
                return Result::Err("Component name not provided!");
            } else {
                break;
            }
        }

        if name_validator.is_some() {
            let name_validator = Regex::new(&name_validator.unwrap()).expect("Invalid regexp.");

            if !name_validator.is_match(&component_name.as_ref().unwrap()) {
                return Result::Err("The component name does not match the provided regex.");
            }
        }

        Result::Ok(Config {
            component_name: component_name.unwrap(),
            generate_all,
            dir: dir.or(Some(String::from("./"))).unwrap(),
        })
    }
}
