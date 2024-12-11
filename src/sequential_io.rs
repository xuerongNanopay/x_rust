use std::path::Path;

fn is_directory_exist(path: &str) -> bool {
    return Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_io() {
        println!("{}", is_directory_exist("./target"));
    }
}
