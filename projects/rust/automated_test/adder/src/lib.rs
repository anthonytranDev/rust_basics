mod shapes;

#[allow(unused_imports)]
use shapes::Rectangle;


#[cfg(test)]
mod tests {
    // Note that we’ve added a new line inside the tests 
    // module: use super::*;. The tests module is a regular
    // module that follows the usual visibility rule
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let medium = Rectangle { width: 7, height: 6 };
        let _smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&medium));
    }
}