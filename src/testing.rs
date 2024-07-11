fn do_work(first: &str, second: &str) -> String{
    
    let pattern = ['#', '@', '€'];
    if first.contains(pattern) | second.contains(pattern) {
        panic!("Invalid input in inputs !")
    }
    
    let mut result = String::new();

    result.push_str(first);
    result.push(' ');
    result.push_str(second);

    result
}


// Code will not be compiled to release
// #[cfg(test)]
mod mytest {
    #![cfg(test)]

    use super::do_work;

    #[test]
    fn test_normal_inputs() {
        let result = do_work("hello", "world");
        assert_eq!(result, "hello world");

        let result = do_work("foo", "bar");
        assert_eq!(result, "foo bar");
    }

    #[test]
    #[should_panic(expected = "Invalid input in inputs !")]
    fn test_first_input_with_invalid_char() {
        do_work("hello#", "world");
    }

    #[test]
    #[should_panic(expected = "Invalid input in inputs !")]
    fn test_second_input_with_invalid_char() {
        do_work("hello", "world@");
    }

    #[test]
    #[should_panic(expected = "Invalid input in inputs !")]
    fn test_both_inputs_with_invalid_chars() {
        do_work("hello#", "world@");
    }

    #[test]
    fn test_empty_strings() {
        let result = do_work("", "");
        assert_eq!(result, " ");
    }
}
