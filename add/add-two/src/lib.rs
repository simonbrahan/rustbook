pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_two() {
        assert_eq!(4, add_two(2));
    }
}
