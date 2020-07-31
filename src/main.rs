extern crate clap;
extern crate itertools;
extern crate regex;

mod cli;
mod fields_delimiters;
mod regex_templates;

use std::io::{self, BufRead};

fn main() {
    let arguments = cli::parse_cli_arguments();

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    if arguments.regex.is_some() {
        regex_templates::run_regex_template_flow(arguments, lines);
    } else {
        fields_delimiters::run_fields_delimiters_flow(arguments, lines);
    }
}
