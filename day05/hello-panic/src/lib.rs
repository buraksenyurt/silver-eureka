#[cfg(test)]
mod tests {
    /*
        Aşağıdaki test fonksiyonunda [should_panic] bildirimi ile test metodundan bilinçli olarak panic üretileceğini belirttik.
        panic!() makrosu çağırıldığında thread anında sonlandırılı ve kodun akan kısmı işletilmez.
        Yani programı açıkça durdurmuş oluyoruz.
    */
    #[test]
    #[should_panic]
    fn panic_test_1() {
        panic!();
        // // any code following this expression is unreachable uyarısı verilir.
        // let x = 10;
        // let y = 20;
        // let z = x + y;
        // assert_eq!(z, 30);
    }

    /*
        get_approver_id metodunda i32 türünden Node değer dönmesi halinde bir panic oluşacağını beklediğimizi söyledik.
        Buna göre aşağıdaki test metodu çalışırken panic ile thread kesilecektir.
        Üstelik panic ile üretilen mesaj "Onaycı bulunamadı." şeklinde beklenmektedir.
    */
    #[test]
    #[should_panic(expected = "Onaycı bulunamadı.")]
    fn panic_test_2() {
        get_approver_id();
    }

    fn get_approver_id() -> i32 {
        None::<i32>.expect("Onaycı bulunamadı.")
    }

    /*
        Sistemi panic ile bilinçli olarak kestiğimiz durumlarda formatlı mesajlar ile daha anlamlı bilgiler verebiliriz.
        Hatta işletim sistemleri için anlamlı olabilecek sayısal değerler de kullanabiliriz.

    */
    #[test]
    #[should_panic(expected = "192.168.1.2 adresine erişilemiyor.")]
    fn panic_test_3() {
        panic!("{} adresine erişilemiyor.", "192.168.1.2");
        // panic!(66); // İşletim sistemine kesme ile ilgili anlamlı bir sayı verip buna göre bir süreç işletmesini de sağlayabiliriz
    }

    use std::f64::consts::PI;

    /*
        assert!, assert_eq! assert_ne! gibi test süreçlerinde sıklıkla kullandığımız makrolar ile de
        thread çalışmasını panic ürettirerek durdurabiliriz.
        Aşağıdaki örnekte should_panic niteliği(attribute) kullanılarak fail eden test sonuçlarında thread'in durdurulması özetlenmektedir.
    */
    #[test]
    #[should_panic]
    fn panic_test_4() {
        let pi = 3.14;
        let real_pi = PI;

        assert!(pi == PI);
        assert_eq!(pi, PI);
        assert_ne!(PI, real_pi);
    }
}
