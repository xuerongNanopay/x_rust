#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::ops::Range;
    use std::fs;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::time::SystemTime;

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

    fn create_files(dir: &str, prefix: &str, n: i32) -> Vec<File> {
        let range = Range {start: 1, end: n+1};
        let mut files = vec!();
        for i in range {
            let file = PathBuf::from(dir).join(format!("{}_{}", prefix, i));
            let f: File = fs::File::create_new(file).unwrap();
            files.push(f);
        }
        files
    }

    fn init_fd() -> Vec<File> {
        create_directory(TMP_DIR);
        create_files(TMP_DIR, "io_test", 10)
    }

    #[test]
    fn sequential_io() {
        let mut fds = init_fd();

        let start = SystemTime::now();
        for fd in fds.iter_mut() {
            for _ in 0..4000000 {
                fd.write(b"Hello, ssss").unwrap();
                fd.flush().unwrap()
            }
        }
        println!("Use {} seconds", start.elapsed().unwrap().as_secs());

    }

    #[test]
    fn random_io() {
        let mut fds = init_fd();

        let start = SystemTime::now();
        for _ in 0..4000000 {
            for fd in fds.iter_mut() {
                fd.write(b"Hello, rrrr").unwrap();
                fd.flush().unwrap()
            }
        }
        println!("Use {} seconds", start.elapsed().unwrap().as_secs());
    }
}
