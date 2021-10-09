#[cfg(test)]
mod tests {
    #[test]
    fn conditionals_test() {
        let point = 85;
        if point < 50 {
            assert!(point < 50);
        } else if point > 50 {
            assert!(point > 50);
        } else {
            assert_eq!(point, 50);
        }

        let scale = Some(2);
        if let Some(value) = scale {
            assert_eq!(value, 2);
        }
    }

    #[test]
    fn conditionals_test2() {
        let mut option = Some(3);
        while let Some(value) = option {
            option = if value > 0 { Some(value - 1) } else { None }
        }
        assert_eq!(option, None);
    }

    #[test]
    fn loops_test() {
        let mut i = 0;
        let mut total = 0;

        loop {
            i += 1;
            if i < 100 {
                total += i;
                continue;
            } else if i >= 100 {
                break;
            }
        }
        assert_eq!(total, 4950);
    }

    #[test]
    fn loops_test2() {
        let mut total = 0;
        for i in 0..100 {
            total += i;
            assert!(i >= 0 && i < 100);
        }
        assert_eq!(total, 4950);
    }

    #[test]
    fn loops_test3() {
        let mut total = 0;
        for p in vec![3, 5, 7, 9, -3, -5, -7, -9].iter() {
            total += p;
        }
        assert_eq!(total, 0);
    }
}
