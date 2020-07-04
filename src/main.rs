extern crate clap;
extern crate regex;

mod cli;

use regex::Regex;

use std::io::{self, BufRead};

// from : https://stackoverflow.com/a/49734423/2234619

fn main() {
    let arguments = cli::parse_cli_arguments();

    let provided_regex = arguments.regex.unwrap();
    let re = Regex::new(&provided_regex).unwrap();

    let input = "2010-03-14";

    let provided_template = arguments
        .template
        .unwrap()
        .replace("{{", "${")
        .replace("}}", "}");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        println!("{}", re.replace_all(&line, provided_template.as_str()));
    }
}
