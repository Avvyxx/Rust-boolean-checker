pub fn is_true(b: bool) -> bool {
    if b == true && b != false && !b == false && !b != true {
        true
    } else {
        false
    }
}

pub fn is_false(b: bool) -> bool {
    if b == false && b != true && !b == true && !b != false {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{is_false, is_true};

    #[test]
    fn test_true_1() {
        assert_eq!(true, is_true(true));
    }

    #[test]
    fn test_true_2() {
        assert_eq!(true, is_true(1 == 1));
    }

    #[test]
    fn test_true_3() {
        assert_ne!(false, is_true(true));
    }

    #[test]
    fn test_true_4() {
        assert_ne!(false, is_true(1 == 1));
    }

    #[test]
    fn test_false_1() {
        assert_eq!(true, is_false(false));
    }

    #[test]
    fn test_false_2() {
        assert_eq!(true, is_false(1 != 1));
    }

    #[test]
    fn test_false_3() {
        assert_ne!(true, is_false(true));
    }

    #[test]
    fn test_false_4() {
        assert_ne!(false, is_false(1 != 1));
    }
}