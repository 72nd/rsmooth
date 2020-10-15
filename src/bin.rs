#[macro_use]
extern crate log;

mod error;
mod file;

use file::File;

use clap::{App, AppSettings, Arg, ArgMatches};

/// Default CLI interface for rsmooth-lib.
pub fn main() {
    let matches = App::new("rsmooth")
        .version("0.2.0")
        .author("72nd <msg@frg72.com>")
        .about("simple tool chain/wrapper for pandoc.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("debug")
                .about("enable debug mode")
                .long("debug")
                .short('d')
                .global(true),
        )
        .arg(
            Arg::new("INPUT")
                .about("path to input markdown file")
                .value_name("INPUT")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .about("optional output file path")
                .long("output")
                .short('o')
                .takes_value(true),
        )
        .subcommand(
            App::new("example")
                .about("outputs a example markdown file with all available header fields"),
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
        Some(("example", x)) => example_cmd(x),
        Some((&_, _)) => {}
        None => default_cmd(&matches),
    }
}

/// Handles example subcommand.
fn example_cmd(_matches: &ArgMatches) {
    error!("not implemented yet")
}

/// Handles default command.
fn default_cmd(matches: &ArgMatches) {
    info!("hoi");
    let file = match File::new(
        matches.value_of("INPUT").unwrap(),
        matches.value_of("output"),
    ) {
        Ok(x) => x,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    match file.convert() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
}
