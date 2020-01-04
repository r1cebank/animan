use std::path::PathBuf;

pub fn process(path: PathBuf, dryrun: bool, name: &str) {
    println!("{:?},{},{}", path, dryrun, name);
}
