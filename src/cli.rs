use clap::crate_version;
use clap::Clap;
use std::process;
use clap::AppSettings::ColoredHelp;

/// The cut clone with regex capabilities.
#[derive(Clap)]
#[clap(version = crate_version!(), setting = ColoredHelp)]
pub struct AppOptions {
    /// Regex expression which specifies capture groups
    #[clap(short, long)]
    pub regex: Option<String>,

    /// String template that specifies how to use the capture group
    #[clap(short, long, allow_hyphen_values = true)]
    pub template: Option<String>,

    /// Delimiter on which to cut the string
    #[clap(short, long, allow_hyphen_values = true)]
    pub delimiter: Option<String>,

    /// Fields
    #[clap(short, long, allow_hyphen_values = true)]
    pub fields: Option<String>,

    /// The string used to join fields, if provided
    #[clap(short, long, allow_hyphen_values = true)]
    pub join_string: Option<String>,

    /// Runs in debug mode
    #[clap(long)]
    pub debug: bool,
}

fn check_arguments(args: &AppOptions) {
    if (args.regex.is_some() || args.template.is_some())
        && (args.regex.is_none() || args.template.is_none())
    {
        println!(
            "Arguments missing! When using values for the regex \
             functionality both the 'template' and 'regex' arguments \
             need to be provided"
        );
        process::exit(1);
    }

    if args.delimiter.is_some() {
        if args.fields.is_none() {
            println!(
                "Arguments missing! When using values for 'fields' of 'delimiter' \
                  you must provide both arguments"
            );
            process::exit(1);
        }

        // now check that the fields are well formatted
        let r = regex::Regex::new(r"^(\d?(\-\d?)?,?)+$").unwrap();
        if !r.is_match(args.fields.clone().unwrap().as_str()) {
            println!(
                "The provided field value is not formatted correctly \
                      It should be composed of a 'comma separated' list where \
                      each element is a range of the shape 'd-d' where either \
                      the closing or opening limit can be optionally provided"
            );
            process::exit(1);
        }
    }
}

pub fn parse_cli_arguments() -> AppOptions {
    let args = AppOptions::parse();
    check_arguments(&args);
    args
}
