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
}
