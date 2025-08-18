pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    // ! this will allow us to use all the imported modules of the outer one
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("This function will fail")
    // }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting should have abdou but it has {}",
            result
        );
    }

    #[test]
    fn more_testing() {
        let result = add(1, 1);
        println!("To see this you must use --show-output");
        assert_ne!(3, result);
    }
}
