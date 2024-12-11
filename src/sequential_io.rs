#[cfg(test)]
mod tests {
    use std::ops::Range;
    use std::fs;
    use std::path::{Path, PathBuf};

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

    const TMP_DIR: &str = "./tmp";

    fn create_files(dir: &str, prefix: &str, n: i32) -> Result<(), Box<dyn std::error::Error>> {
        let range = Range {start: 1, end: n+1};

        for i in range {
            let file = PathBuf::from(dir).join(format!("{}_{}", prefix, i));
            fs::File::create_new(file)?;
        }
        Ok(())
    }

    fn init() {
        create_directory(TMP_DIR);
        create_files(TMP_DIR, "io_test", 50).unwrap();
    }

    #[test]
    fn sequential_io() {
        init();
    }

    fn random_io() {

    }
}
