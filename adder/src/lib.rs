pub fn add(num: i32) -> i32 {
    num + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be 1 and 100, got {}.", value)
        }

        Guess { value }
    }
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Guess value must be 1 and 100")]
    fn greater_than_100() {
        Guess::new(0);
    }

    #[test]
    fn test_can_hold() {
        let rec1: Rectangle = Rectangle { width: 8, height: 7 };
        let rec2: Rectangle = Rectangle { width: 5, height: 1 };

        let rec1_can_hold_rec2 = rec1.can_hold(&rec2);
        assert!(rec1_can_hold_rec2)
    }

    #[test]
    fn test_cannot_hold() {
        let rec1: Rectangle = Rectangle { width: 8, height: 7 };
        let rec2: Rectangle = Rectangle { width: 5, height: 1 };

        let rec2_cannot_hold_rec1 = rec2.can_hold(&rec1);
        assert!(!rec2_cannot_hold_rec1, "Failed here")        
    }

}
