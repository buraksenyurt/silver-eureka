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

    #[test]
    fn should_webby_works() {
        webby!(GET "https://www.buraksenyurt.com" -> { Response{code:200} });
        webby!(POST "https://someservice/product/update" -> {Response{code:200}});
    }

    #[test]
    fn should_smartsum_works_for_n_times() {
        let mut value = 0;
        // smartsum isimli makro son parametre olarak gelen kod bloğunu ilgili değer aralığı kadar çalıştırır.
        // Mantıksız bir örnek oldu tabii.
        smartsum!(from 5 to 10, {
            value += 5;
        });
        assert_eq!(value, 25);
    }

    #[test]
    fn should_makefunc_create_function() {
        /*
        makefunc makrosu üç parametre almakta. İlki fonksiyon adı.
        İkincisi işler bir kod bloğun ve üçüncüsü de fonksiyondan dönecek olan tür.
        Makromuz bu argümanlara göre bloğu bir fonksiyon haline getirmekte.
        Sonrasında onu çalıştırabiliyoruz.
        */
        makefunc!(
            summmy,
            {
                let mut value = 0;
                for i in 0..100 {
                    value += i
                }
                return value;
            },
            i32
        );
        let result = summmy();
        assert_eq!(result, 4950);
    }
}

/*/
    Makroları modül içerisinde de yazabiliriz.
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

    ///
    /// from 2 to 5 {value+=1} gibi bir ifadenin çalışmasını sağlayan dummy macro!
    ///
    /// ```
    /// let mut value = 0;
    /// hello_macros::smartsum!(from 5 to 10, {
    ///     value += 5;
    /// });
    /// assert_eq!(value, 25);
    /// ```
    #[macro_export]
    macro_rules! smartsum {
        /*
            Bu makro from 2 to 5 { // kodlar } gibi bir ifadenin çalıştırılabilmesini sağlar.
            $f block türünden olduğu için bir kod bloğunu makro argümanı olarak kullanabiliriz.
        */
        (from $first:tt to $last:tt, $b:block) => {
            // first ve last argümanları değer aralığında çalışacak bir döngü
            for _ in $first..$last {
                $b() // parametre olarak gelen kod bloğu çalıştırılır.
            }
        };
    }

    ///
    /// Bir kod bloğunu verilen isim dönüş türüne göre fonksiyon haline getiren dummy macro.
    ///
    /// ```
    /// hello_macros::makefunc!(
    ///         summmy,
    ///         {
    ///             let mut value = 0;
    ///             for i in 0..100 {
    ///                 value += i
    ///             }
    ///             return value;
    ///         },
    ///         i32
    ///         );
    /// let result = summmy();
    /// assert_eq!(result, 4950);
    /// ```
    #[macro_export]
    macro_rules! makefunc {
        /*
            ident ile metod adının da oluğu tanımlama kısmını ifade ediyoruz.
            block türü ile kod bloğu alınıyor.
            tt ile de single token tree söz konusu. Bir dönüş türü için ele alıyoruz.
        */
        ($identity:ident,$body:block,$return:tt) => {
            // aşağıdaki fonksiyona ait bloğun yazılışı icra edilmekte.
            fn $identity () -> $return $body
        };
    }

    ///
    /// Sembolik olarak bir kaynağa HTTP talebi atan dummy macro'dur.
    ///
    /// ```
    /// use hello_macros::send;
    ///
    /// hello_macros::webby!(GET "https://www.buraksenyurt.com" -> { hello_macros::Response{code:200} });
    /// hello_macros::webby!(POST "https://someservice/product/update" -> {hello_macros::Response{code:200}});
    /// ```
    ///
    #[macro_export]
    macro_rules! webby {
        /*
           Bu kez dört farklı bacak var. Sembolik olarak bir web adresine paket göndermekte yardımcı olacak kodları hazırlayan bir makro olarak düşünelim.
           Aranan eşleşmelerde GET gibi bir HTTP metodu ile başlanmakta. Ardından web adresine ait yol bilgisi alınıyor.
           -> sembolünü takiben bir kod bloğu beklediğimizi ifade ediyoruz. Bu argümanlar send metoduna parametre olarak yollanıyorlar.

        */
        (GET $path:literal -> $b:block) => {
            send("GET", $path, &|| $b)
        };
        (POST $path:literal -> $b:block) => {
            send("POST", $path, &|| $b)
        };
        (PUT $path:literal -> $b:block) => {
            send("PUT", $path, &|| $b)
        };
        (DELETE $path:literal -> $b:block) => {
            send("DELETE", $path, &|| $b)
        };
    }
}

pub struct Response {
    pub code: usize,
}
pub fn send(_method: &str, _path: &str, _handler: &dyn Fn() -> Response) {}
