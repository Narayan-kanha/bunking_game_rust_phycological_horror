use std::fs;
use std::path::Path;

pub fn ensure_dir(path: &str) {
    let p = Path::new(path);
    if !p.exists() {
        if let Err(e) = fs::create_dir_all(p) {
            eprintln!("Failed to create dir {}: {}", path, e);
        }
    }
}
