pub struct Guess {
    pub value: i32,
}

impl Guess {
    #[allow(dead_code)]
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }
    #[test]
    fn inbetween_1_and_100() {
        Guess::new(50);
    }
}