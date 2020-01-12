use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub fn process(path: PathBuf, dryrun: bool, name: &str) {
    let entries = fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|path| path.path().is_file())
        .collect::<Vec<_>>();

    let re = Regex::new(r"^.*\[(?P<num>\d{2})\].*$").unwrap();
    let new_names = entries
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
    println!("{:?}", new_names)
    // for entry in entries.iter() {
    //     let file_name = entry.file_name();
    //     // let new_name = re.replace_all(file_name, "Anime - s1e$num.mp4");
    //     println!(
    //         "{:?}\n",
    //         (file_name.to_str(), re.captures(file_name.to_str().unwrap()))
    //     );
    // }
}
