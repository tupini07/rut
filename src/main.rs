extern crate clap;
extern crate itertools;
extern crate regex;

mod cli;

use regex::Regex;

use itertools::Itertools;
use std::io::{self, BufRead};

// from : https://stackoverflow.com/a/49734423/2234619
fn get_fields(text: Vec<String>, field_statements: Vec<&str>, join_string: String) {
    dbg!(&text);
    dbg!(&field_statements);

    let sep_string = Box::new(join_string);

    let mut res = String::new();

    for f in field_statements {
        if !res.is_empty() {
            res = format!("{}{}", res, sep_string);
        }

        if !f.contains("-") {
            res.push_str(text.iter().nth(f.parse::<usize>().unwrap()).unwrap());
        } else {
            if f.starts_with("-") {
                // open at start
                let r = Regex::new(r"^\-(\d)$").unwrap();
                let p = r.replace_all(f, "$1").parse::<usize>().unwrap();

                res = format!(
                    "{}{}",
                    res,
                    text.iter()
                        .take(p)
                        .cloned()
                        .intersperse(sep_string.to_string())
                        .collect::<String>()
                )
            } else if f.ends_with("-") {
                //open at close
                let r = Regex::new(r"^(\d)\-$").unwrap();
                let rep = r.replace_all(f, "$1");
                dbg!(&rep);
                let p = rep.parse::<usize>().unwrap();

                res = format!(
                    "{}{}",
                    res,
                    text.iter()
                        .skip(p - 1)
                        .cloned()
                        .intersperse(sep_string.to_string())
                        .collect::<String>()
                )
            } else {
                //specifies a range of both close and end
                let r = Regex::new(r"^(\d)\-(\d)$").unwrap();
                let rep = r
                    .replace_all(f, "$1,$2")
                    .split(",")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let (l1, l2): (usize, usize) = (rep[0] - 1, rep[1]);

                res = format!(
                    "{}{}",
                    res,
                    text.iter()
                        .skip(l1)
                        .take(l2 - l1)
                        .cloned()
                        .intersperse(sep_string.to_string())
                        .collect::<String>()
                )
            }
        }
    }

    dbg!(&res);

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
