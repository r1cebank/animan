use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::io;

pub fn process(path: PathBuf, dryrun: bool, name: &str) {
    let path_name = path.to_str().unwrap();
    let entries = fs::read_dir(PathBuf::from(path_name))
        .unwrap()
        .filter_map(Result::ok)
        .filter(|path| path.path().is_file())
        .collect::<Vec<_>>();

    let re = Regex::new(r"^.*\[(?P<num>\d{2})\].*$").unwrap();
    let file_names = entries
        .iter()
        .filter(|entry| {
            let file_name = entry.file_name();
            let matches = re.captures(file_name.to_str().unwrap());
            match matches {
                Some(_) => true,
                None => false,
            }
        })
        .collect::<Vec<_>>();
    println!("Found {} matching files.", file_names.len());
    let new_names = file_names
        .iter()
        .map(|entry| {
            let file_name = entry.file_name();
            let matches = re.captures(file_name.to_str().unwrap()).unwrap();
            format!(
                "{} - s1e{}.{}",
                name,
                &matches["num"],
                entry.path().extension().unwrap().to_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    if dryrun {
        for (i, name) in file_names.iter().enumerate() {
            println!("{} -> {}", name.file_name().to_str().unwrap(), new_names[i]);
        }
    } else {
        for (i, name) in file_names.iter().enumerate() {
            match io::rename(
                PathBuf::from(path_name),
                name.file_name().to_str().unwrap(),
                new_names[i].as_str(),
            ) {
                Ok(_) => println!(" ✓"),
                Err(_) => println!(" x"),
            }
        }
    }
}
