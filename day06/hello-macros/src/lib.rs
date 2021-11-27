#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_sum_works_for_two_number() {
        let result = sum!(3.1, 4.5, f32);
        assert_eq!(result, 7.6);

        let result = sum!(8, 9, u8);
        assert_eq!(result, 17);
    }
}

/*/
    Makroları modül içerisinde de yapabiliriz.
*/
#[macro_use]
mod macromania {
    // modül dışında sum makrosunun kullanılabilmesi için macro_export direktifi kullanılır.    
    // macro_rules! ile declartive tipte makrolar tanımlanabilir. Birde procedural macro'lar var.

    ///
    /// İki değerin belirtilen türde toplanmasını sağlayan dummy macro.
    /// 
    /// ```
    /// let result = hello_macros::sum!(3,4,u8);
    /// assert_eq!(result,7);
    /// ```
    /// 
    #[macro_export]
    macro_rules! sum {
        // Aşağıdaki ifadede & ile başlayan yerlerde sentaks enstrümanları işaret edilir.
        // expr bir expression olduğunu, ty ise bir tip oldğu ifade eder.
        ($a:expr,$b:expr,$t:ty) => {
            // a ve b ifadelerinin t türüne dönüştürülerek toplanması işlemi söz konusudur.
            $a as $t + $b as $t
        };
    }
}
