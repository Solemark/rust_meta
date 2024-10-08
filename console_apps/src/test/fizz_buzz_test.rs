#[cfg(test)]
mod tests {
    use crate::fizz_buzz::FizzBuzz;

    #[test]
    fn test_fizz_buzz() {
        let result: String = FizzBuzz::new(3, 5, 20).play();
        let expect: String = String::from("1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizzbuzz, 16, 17, fizz, 19, buzz");
        assert_eq!(result, expect);
    }
}
