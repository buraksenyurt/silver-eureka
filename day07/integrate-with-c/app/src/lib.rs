use std::ffi::{CStr, CString}; //  FFI (Foreign Function Interface) kütüphanesinden kullanacaklarımız.
use std::os::raw::{c_char, c_void}; // kriptografi paketi.

use ring::digest;
/*
    C tarafından bağlayacağımız kütüphaneden kullanılacak fonksiyonlar için bir arayüz tanımladık diyebiliriz.
    C ile yazılacak kodlarda pre_digest ve post_digest isimli parametre almayıp değer döndürmeyen (void) iki fonksiyon söz konusu olacak.
*/
extern "C" {
    fn pre_encrypt() -> c_void;
    fn post_encrypt() -> c_void;
}

/*
    encrypt fonksiyonu C kütüphanesinden çağırılacak bir fonksiyon.
    Bu sebeple C isimlanına harici bir bağlantı olarak tanımlanıyor(extern "C")
    mutable yani içeriği değiştirilebilir c_char türünden bir değişken alıp yine aynı türden geriye döndürüyor.
*/
#[no_mangle] // C tarafı ile daha kolay bağlanabilmek için Rust'ın isimlendirme yönetiminin kapatılması öneriliyor
pub extern "C" fn encrypt(data: *mut c_char) -> *mut c_char {
    // Dışarıdan çağırılacak her türlü kod güvensizdir.
    unsafe {
        pre_encrypt(); // C kütüphanesinden çağırılacak fonksiyon.

        let data = CStr::from_ptr(data); // input olarak gelen referanstan Rust'ın kullanacağı String türüne dönüştürme yapılır.
        let sign = digest::digest(&digest::SHA256, data.to_bytes()); // SHA256 türünden kriptolama yapılan yer.
        let hex_data = sign
            .as_ref()
            .iter()
            .map(|c| format!("{:X}", c))
            .collect::<String>(); // kriptolanan içeriği karakter bazında hexadecimal forma dönüştürülür.

        post_encrypt(); // C kütüphanesinden çağırılacak diğer fonksiyon.

        CString::new(hex_data).unwrap().into_raw() // İçerik C tarafının istediği String türüne evrilir.
    }
}

#[test]
fn should_encrypt_sha256_works() {
    let sign = digest::digest(&digest::SHA256, b"Rustgele dostlar!");
    let hex_data = sign
        .as_ref()
        .iter()
        .map(|c| format!("{:X}", c))
        .collect::<String>();

    assert_eq!(
        "17305F12ACF3436C2CC6D249BECD3AA419727B442F4A542ADEC5AA6F1C12",
        hex_data
    );
}
