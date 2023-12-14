fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_or_odd() {
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(7), "Odd");
        assert_eq!(even_or_odd(1), "Odd");
    }

    #[test]
    fn test_negative_even_or_odd() {
        assert_eq!(even_or_odd(-2), "Even");
        assert_eq!(even_or_odd(-0), "Even");
        assert_eq!(even_or_odd(-7), "Odd");
        assert_eq!(even_or_odd(-1), "Odd");
    }
}