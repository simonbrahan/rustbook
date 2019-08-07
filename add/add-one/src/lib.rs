pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_one() {
        assert_eq!(3, add_one(2));
    }
}
