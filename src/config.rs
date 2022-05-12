use std::{collections::HashMap, env};

const DIR_ARG: &str = "-d";
const ALL_ARG: &str = "-a";

#[derive(Debug)]
pub struct Config {
    pub component_name: String,
    pub generate_all: bool,
    pub dir: String,
    pub extra_args: HashMap<String, String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        args.next(); // skip binary path

        let mut component_name: Option<String> = None;
        let mut generate_all = false;
        let mut dir: Option<String> = None;
        let mut extra_args = HashMap::new();

        loop {
            if let Some(arg) = args.next() {
                match arg.as_str() {
                    DIR_ARG => dir = Some(args.next().unwrap()),
                    ALL_ARG => generate_all = true,
                    extra_arg if extra_arg.starts_with("-") => {
                        extra_args.insert(
                            extra_arg.trim_start_matches("-").to_string(),
                            args.next()
                                .expect(&format!("No value provided for: {} option.", extra_arg)),
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
                return Result::Err(String::from("Component name not provided."));
            } else {
                break;
            }
        }

        return Result::Ok(Config {
            component_name: component_name.unwrap(),
            generate_all,
            extra_args,
            dir: dir.or(Some(String::from("./"))).unwrap(),
        });
    }
}
