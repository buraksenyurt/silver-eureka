#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn test_for_arrays() {
        let mut numbers: [usize; 4] = [1; 4]; // 4 tane 1 sayısından oluşan bir dizi tanımı. mutable.
        assert_eq!(numbers, [1, 1, 1, 1]);
        numbers[0] = 0;
        assert_eq!(numbers, [0, 1, 1, 1]);
        let numbers_copy: [usize; 4] = [3, 5, 7, 9];
        assert_eq!(numbers_copy, [3, 5, 7, 9]);
        assert_eq!(mem::size_of_val(&numbers), mem::size_of::<usize>() * 4); // unsigned int üstünden boyut kontrolü. dizinin boyutu usize ile eleman sayısının çarpımını eşittir
    }

    #[test]
    fn test_for_tuples() {
        let mut basket: (f32, usize, String, i16) = (3.14, 32, "Rust Cookbook".to_string(), -48);
        assert_eq!(basket.3, -48);
        assert_eq!(basket.2, "Rust Cookbook");
        basket.0 = 3.1415;
        assert_eq!(basket.0, 3.1415);

        let (_item1, _item2, _item3, _item4) = basket;
        assert_eq!(_item1, 3.1415);

        let complex1 = Complex(3, 4);
        assert_eq!(complex1.0, 3);
        assert_eq!(complex1.1, 4);
        let (_real, _virtual) = (complex1.0, complex1.1);
        assert_eq!(_real, 3);
        assert_eq!(_virtual, 4);
    }

    struct Complex(i32, i32);
}
