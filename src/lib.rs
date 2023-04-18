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
}