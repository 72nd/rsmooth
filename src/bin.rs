#[macro_use]
extern crate log;

/// Format argument for a PDF output.
const PDF_FORMAT: &str = "pdf";
/// Format argument for a ODT (OpenDocument Text) output.
const ODT_FORMAT: &str = "odt";

use clap::{App, AppSettings, Arg, ArgMatches, ValueHint};

/// Default CLI interface for rsmooth-lib.
pub fn main() {
    let matches = App::new("rsmooth")
        .version("0.3.0")
        .author("72nd <msg@frg72.com>")
        .about("simple tool chain/wrapper for pandoc.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::new("debug")
                .about("enable debug mode")
                .long("debug")
                .short('d')
                .global(true),
        )
        .arg(
            Arg::new("format")
                .about("set the output format")
                .long("format")
                .short('f')
                .takes_value(true)
                .possible_value(PDF_FORMAT)
                .possible_value(ODT_FORMAT)
                .default_value(PDF_FORMAT)
        )
        .arg(
            Arg::new("INPUT")
                .about("path to input markdown file")
                .value_name("INPUT")
                .required(true)
                .index(1)
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("raw")
                .about("outputs the finalized pandoc input on the stdout for debugging")
                .long("raw")
                .short('r')
                .global(true),
        )
        .arg(
            Arg::new("output")
                .about("optional output file path")
                .long("output")
                .short('o')
                .takes_value(true),
        )
        .subcommand(
            App::new("example-file")
                .about("outputs a example markdown file with all available header fields")
                .arg(
                    Arg::new("output")
                        .about("optional output file path")
                        .long("output")
                        .short('o')
                        .takes_value(true),
                ),
        )
        .get_matches();

    env_logger::Builder::new()
        .filter(
            None,
            match matches.is_present("debug") {
                true => log::LevelFilter::Debug,
                false => log::LevelFilter::Info,
            },
        )
        .init();

    match matches.subcommand() {
        Some(("example-file", x)) => example_cmd(x),
        Some((&_, _)) => {}
        None => default_cmd(&matches),
    }
}

/// Handles example subcommand.
fn example_cmd(matches: &ArgMatches) {
    match lib::example(matches.value_of("output")) {
        Ok(x) => match x {
            Some(x) => println!("{}", x),
            None => {}
        },
        Err(e) => error!("{}", e),
    }
}

/// Handles default command.
fn default_cmd(matches: &ArgMatches) {
    match lib::convert(
        matches.value_of("INPUT").unwrap(),
        matches.value_of("output"),
        matches.is_present("raw"),
        match matches.value_of("format").unwrap() {
            PDF_FORMAT => lib::OutputFormat::Pdf,
            ODT_FORMAT => lib::OutputFormat::Odt,
            _ => lib::OutputFormat::Pdf,
        },
    ) {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };
}
