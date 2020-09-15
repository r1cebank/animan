use std::fs;
use std::path::PathBuf;

pub fn rename(base_dir: PathBuf, original: &str, new: &str) -> std::io::Result<()> {
    let original_path = base_dir.join(original);
    let new_path = base_dir.join(new);
    print!("{} -> {}", original, new);
    fs::rename(original_path, new_path)
}
