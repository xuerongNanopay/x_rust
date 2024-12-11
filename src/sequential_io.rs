use std::path::Path;
use std::fs;

fn is_directory_exist(path: &str) -> bool {
    return Path::new(path).exists()
}

fn rm_directory(path: &str) {
    if is_directory_exist(path) {
        fs::remove_dir_all(path).unwrap()
    }
}

fn create_directory(path: &str) {
    if !is_directory_exist(path) {
        fs::create_dir(path).unwrap()
    } else {
        fs::remove_dir_all(path).unwrap();
        fs::create_dir(path).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TMP_DIR: &str = "./tmp";

    fn initial_dir() {
        create_directory(TMP_DIR)
    }

    fn clear_dir() {
        rm_directory(TMP_DIR)
    }

    #[test]
    fn sequential_io() {
        create_directory(TMP_DIR)
    }
}
