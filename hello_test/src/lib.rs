#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[should_panic]
    #[test]
    fn another() {
        panic!("make this test fail")
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 2,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, values was `{}`",
            result
        );
    }

    #[should_panic]
    #[test]
    fn greater_than_100(){
        Guess::new(123);
    }

    #[ignore]
    #[test]
    fn this_test_will_pass(){
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello, {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value <1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {
            value:value,
        }
    }
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
