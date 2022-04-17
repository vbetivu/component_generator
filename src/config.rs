use std::{collections::HashMap, env};

use regex::Regex;

use crate::constants::{ALL_ARG, DIR_ARG, REGEX_ARG};

#[derive(Debug)]
pub struct Config {
    pub component_name: String,
    pub extra_args: HashMap<String, String>,
    pub generate_all: bool,
    pub dir: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        // skip binary path
        args.next();

        let mut component_name = Option::None;
        let mut extra_args = HashMap::new();
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
                    extra_arg if extra_arg.starts_with("-") => {
                        extra_args.insert(
                            extra_arg.trim_start_matches("-").to_string(),
                            args.next().expect(&format!(
                                "No value provided for: {} argument.",
                                extra_arg.clone()
                            )),
                        );
                    }

                    _ => {
                        if component_name.is_none() {
                            component_name = Some(arg)
                        } else {
                            println!("Unknown argument: \"{}\"", arg);

                            return Result::Err(format!("Invalid argument: \"{}\"", arg));
                        }
                    }
                }
            } else if component_name.is_none() {
                return Result::Err(String::from("Component name not provided!"));
            } else {
                break;
            }
        }

        if name_validator.is_some() {
            let name_validator = Regex::new(&name_validator.unwrap()).expect("Invalid regexp.");

            if !name_validator.is_match(&component_name.as_ref().unwrap()) {
                return Result::Err(String::from(
                    "The component name does not match the provided regex.",
                ));
            }
        }

        Result::Ok(Config {
            component_name: component_name.unwrap(),
            extra_args,
            generate_all,
            dir: dir.or(Some(String::from("./"))).unwrap(),
        })
    }
}
