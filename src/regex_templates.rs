
use regex::Regex;
use std::io::{Lines, StdinLock};
use crate::cli::AppOptions;

pub fn run_regex_template_flow(arguments: AppOptions, lines: Lines<StdinLock>) {
    let provided_regex = arguments.regex.unwrap();
    let re = Regex::new(&provided_regex).unwrap();

    let provided_template = arguments
        .template
        .unwrap()
        .replace("{{", "${")
        .replace("}}", "}");

    for line in lines {
        let line = line.expect("Could not read line from standard in");
        println!("{}", re.replace_all(&line, provided_template.as_str()));
    }
}
