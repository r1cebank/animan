extern crate clap;

use clap::{App, Arg};
use std::env;
use std::path::PathBuf;

mod io;
mod process;

fn main() {
    let mut path = match env::current_dir() {
        Ok(p) => p,
        Err(_) => panic!("Error accessing current directory"),
    };
    let path_name: PathBuf;
    let current_dir = env::current_dir().unwrap();
    let current_dir_name = current_dir.file_name();
    let mut name = match current_dir_name {
        Some(n) => n.to_str().unwrap(),
        None => "",
    };
    let mut dryrun = false;
    let matches = App::new("animan")
        .version("0.1.0")
        .author("Siyuan Gao <siyuangao@pm.me>")
        .arg(
            Arg::with_name("dryrun")
                .short("d")
                .long("dryrun")
                .help("Dry-run the rename operations"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Overrides the path of the dump"),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Override the series name"),
        )
        .get_matches();

    if matches.is_present("path") {
        path = PathBuf::from(matches.value_of("path").unwrap());
        path_name = PathBuf::from(matches.value_of("path").unwrap());
        name = path_name.file_name().unwrap().to_str().unwrap();
    }
    if matches.is_present("dryrun") {
        dryrun = true;
    }
    if matches.is_present("name") {
        name = matches.value_of("name").unwrap();
    }
    // Read each args, set the series name from directory name or input
    process::process(path, dryrun, name);
}
