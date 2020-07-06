extern crate clap;
extern crate itertools;
extern crate regex;

mod cli;

use regex::Regex;

use itertools::Itertools;
use std::io::{self, BufRead};

fn get_fields(text: Vec<String>, field_statements: Vec<&str>, join_string: String) {
    let sep_string = Box::new(join_string);

    let mut res = String::new();

    let get_limits = |r: &str, l1: usize, l2: usize| {
        let end = if l2 > 0 {
            l2
        } else {
            text.iter().len()
        };

        format!(
            "{}{}",
            r,
            text.iter()
                .skip(l1)
                .take(end)
                .cloned()
                .intersperse(sep_string.to_string())
                .collect::<String>()
        )
    };

    for f in field_statements {
        if !res.is_empty() {
            res = format!("{}{}", res, sep_string);
        }

        if !f.contains("-") {
            // then we're being requested a single field
            res.push_str(text.iter().nth(f.parse::<usize>().unwrap()).unwrap());
        } else {
            if f.starts_with("-") {
                // open at start
                let r = Regex::new(r"^\-(\d)$").unwrap();
                let p = r.replace_all(f, "$1").parse::<usize>().unwrap();
                res = get_limits(&res, 0, p);
            } else if f.ends_with("-") {
                //open at close
                let r = Regex::new(r"^(\d)\-$").unwrap();
                let rep = r.replace_all(f, "$1");
                let p = rep.parse::<usize>().unwrap();
                res = get_limits(&res, p-1, 0);
            } else {
                //specifies a range of both close and end
                let r = Regex::new(r"^(\d)\-(\d)$").unwrap();
                let rep = r
                    .replace_all(f, "$1,$2")
                    .split(",")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let (l1, l2): (usize, usize) = (rep[0] - 1, rep[1]);

                res = get_limits(&res, l1, l2 - l1);
            }
        }
    }

    println!("{}", res);
}

fn main() {
    let arguments = cli::parse_cli_arguments();

    if arguments.regex.is_some() {
        let provided_regex = arguments.regex.unwrap();
        let re = Regex::new(&provided_regex).unwrap();

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
    } else {
        // TODO: refactor this and send text that is being read from stdin
        // instead of using a static string
        let text = "some text -- that is -- separated".to_string();
        let text_vec: Vec<String>;

        // if we were provided a delimiter then split on it, else split on every character
        if arguments.delimiter.is_some() {
            text_vec = text
                .as_str()
                .split(&arguments.delimiter.unwrap())
                .map(|e| e.to_string())
                .collect();
        } else {
            text_vec = text.chars().map(|e| e.to_string()).collect();
        }

        get_fields(
            text_vec,
            arguments.fields.unwrap().as_str().split(",").collect(),
            arguments.join_string.unwrap_or(String::new()),
        )
    }
}
