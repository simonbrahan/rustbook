#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigger_can_hold_smaller() {
        let bigger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(bigger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_bigger() {
        let bigger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&bigger));
    }

    #[test]
    fn can_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("simon");
        assert!(
            result.contains("simon"),
            "Greeting did not contain name, value was: {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess must be between 1 and 100")]
    fn guess_greater_than_100() {
        Guess::new(101);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 did not equal 4"))
        }
    }
}
