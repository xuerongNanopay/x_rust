#[warn(dead_code)]
fn aaa(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_io() {
        let result = aaa(2, 2);
        assert_eq!(result, 4);
    }
}
