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

    #[test]
    fn should_sumall_works_for_numbers() {
        let result = sumall!(1, 2, 3, 4, 5, 6);
        assert_eq!(result, 21);

        let result = sumall!();
        assert_eq!(result, 0);
    }

    #[test]
    fn should_funmulti_works_for_numbers() {
        let result = funmulti!(4, 5);
        assert_eq!(result, 20);

        let result = funmulti!(1, 2, 3, 4, 5, 6);
        assert_eq!(result, 720);
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

    ///
    /// 0 veya n sayıda değeri ardışıl olarak toplayan dummy macro.
    ///
    /// ```
    /// let result = hello_macros::sumall!(1,2,3);
    /// assert_eq!(result,6);
    ///
    /// let result = hello_macros::sumall!();
    /// assert_eq!(result,0);
    /// ```
    #[macro_export]
    macro_rules! sumall {
        /*
            Aşağıdaki eşleştirmeye dikkat edelim.
            a bir ifade olaraktan aralarında virgül bulunan ve * sebebiyle de n sayıda olabilecek bir sentaksı yakalıyoruz.
            Buna uygun bir sentaks varsa {} içerisine alınan kod bloğu oluşturuluyor.
        */
        ($($a:expr),*)=>{
            {
                // Hiç argüman yoksa 0 dönülecek
                0
                // Eğer n sayıda argüman varsa aralarına + işaret konularaktan sentaks tamamlanacak.
                $(+$a)*
            }
        }
    }

    ///
    /// İki veya n adet sayıyı ardışıl olarak çarpan dummy makrodur.
    ///
    /// ```
    /// let result = hello_macros::funmulti!(4, 5);
    /// assert_eq!(result, 20);
    ///
    /// let result = hello_macros::funmulti!(4, 5);
    /// assert_eq!(result, 20);
    /// ```
    /// 
    #[macro_export]
    macro_rules! funmulti {
        /*
            tt, single token tree anlamındadır.
            Oluşturulan ifadede funsum makrosu kendisini recursive olarak da çağırmaktadır.
            İki durum ele alınır. İlki iki değer girildiğinde toplamını bulan ifadeyi oluşturur.
            İkinci durum da bir a ifadesi ve takip eden n sayıda b ifadesini işaret eden bir token tree söz konusudur.
            *, n tane anlamına gelir.
        */
        ($a:expr,$b:expr) => {
            $a * $b
        };
        ($a:expr,$($b:tt)*)=>{
            $a * funmulti!($($b)*)
        }
    }
}
