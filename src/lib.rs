fn is_true(b: bool) -> bool {
    if b == true && b != false && !b == false && !b != true {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_true;

    #[test]
    fn test_true_1() {
        assert_eq!(true, is_true(true));
    }

    #[test]
    fn test_true_2() {
        assert_eq!(true, is_true(1 == 1));
    }

    #[test]
    fn test_false_1() {
        assert_eq!(false, is_true(false));
    }

    #[test]
    fn test_false_2() {
        assert_eq!(false, is_true(1 != 1));
    }
}