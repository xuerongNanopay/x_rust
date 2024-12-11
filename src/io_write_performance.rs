#[cfg(test)]
mod tests {
    use std::io::{Seek, Write, SeekFrom};
    use std::ops::Range;
    use std::fs;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::time::SystemTime;
    use rand::{seq::SliceRandom, thread_rng};

    const SIZE_IN_BYTE: usize = 250*1024;
    // const SIZE_IN_BYTE: usize = 10;
    const TMP_DIR: &str = "./tmp";

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

    // fn init_fds() -> Vec<File> {
    //     create_directory(TMP_DIR);
    //     create_files(TMP_DIR, "io_test", 10)
    // }

    fn init_fd() -> File {
        create_directory(TMP_DIR);
        create_files(TMP_DIR, "io_test", 1).remove(0)
    }

    fn fill_fd() -> File {
        let mut fd = init_fd();
        let bytes: [u8; SIZE_IN_BYTE] = [48u8; SIZE_IN_BYTE];
        for _ in 0..10 {
            fd.write(&bytes).unwrap();
        }
        fd.seek(SeekFrom::Start(0)).unwrap();
        fd
    }

    #[test]
    fn sequential_io() {
        let mut fd = fill_fd();

        let start = SystemTime::now();
        for _ in 0..SIZE_IN_BYTE {
            fd.write(b"HelloWorld").unwrap();
            fd.flush().unwrap()
        }
        println!("Elapsed: {}", start.elapsed().unwrap().as_millis());

    }

    #[test]
    fn random_io() {
        let mut fd = init_fd();
        // let pos: [u64; SIZE_IN_BYTE] = [0; SIZE_IN_BYTE];
        let mut pos: Vec<u64> = (0..(SIZE_IN_BYTE as u64)).collect();
        pos.shuffle(&mut thread_rng());
        for p in pos.iter_mut() {
            *p = *p * 10u64;
        }
        
        let start = SystemTime::now();
        for p in pos.iter() {
            fd.seek(SeekFrom::Start(*p)).unwrap();
            fd.write(b"HelloWorld").unwrap();
            fd.flush().unwrap()
        }
        println!("Elapsed: {}", start.elapsed().unwrap().as_millis());
    }
}
