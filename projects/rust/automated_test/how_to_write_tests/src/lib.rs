mod arithmetic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() -> Result<(), String> {
        if arithmetic::add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))            
        }
    }
}