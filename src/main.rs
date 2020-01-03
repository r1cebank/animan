extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("animan")
        .version("0.1.0")
        .author("Siyuan Gao <siyuangao@gmail.com>")
        .arg(
            Arg::with_name("dryrun")
                .short("d")
                .long("dryrun")
                .help("Dry-run the rename operations"),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Override the series name"),
        )
        .get_matches();
    println!("{:?}", matches)
}
