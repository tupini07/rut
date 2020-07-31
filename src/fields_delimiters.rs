use crate::cli::AppOptions;
use itertools::Itertools;
use regex::Regex;
use std::io::{Lines, StdinLock};

pub fn get_fields(text: Vec<String>, field_statements: &Vec<&str>, join_string: &String) {
    let mut res = String::new();

    let get_limits = |r: &str, l1: usize, l2: usize| {
        format!(
            "{}{}",
            r,
            text.iter()
                .skip(l1)
                .take(if l2 > 0 { l2 } else { text.iter().len() })
                .cloned()
                .intersperse(join_string.to_string())
                .collect::<String>()
        )
    };

    for f in field_statements {
        if !res.is_empty() {
            res = format!("{}{}", res, join_string);
        }

        if !f.contains("-") {
            // then we're being requested a single field
            let index = f.parse::<usize>().unwrap();
            res.push_str(
                text.iter()
                    .nth(index - 1)
                    .expect(&format!("The provided index '{}' is out of bounds", index)),
            );
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
                res = get_limits(&res, p - 1, 0);
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

pub fn run_fields_delimiters_flow(arguments: AppOptions, lines: Lines<StdinLock>) {
    let delimiter = arguments.delimiter.unwrap_or("".to_string());

    let raw_fields = arguments.fields.unwrap();
    let fields = raw_fields.as_str().split(",").collect::<Vec<&str>>();

    let join_string = arguments.join_string.unwrap_or(String::new());

    for line in lines {
        let line = line.expect("Could not read line from standard in");

        let text_vec: Vec<String>;

        // if we were provided a delimiter then split on it, else split on every character
        if !delimiter.is_empty() {
            text_vec = line
                .as_str()
                .split(&delimiter)
                .map(|e| e.to_string())
                .collect();
        } else {
            text_vec = line.chars().map(|e| e.to_string()).collect();
        }

        get_fields(text_vec, &fields, &join_string)
    }
}
