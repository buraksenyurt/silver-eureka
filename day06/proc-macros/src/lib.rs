extern crate proc_macro;
use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/*
    Attribute tabanlı makrolarda proc_macro_attribute niteliği ile imzalanan fonksiyonlar söz konusudur.
    Bu fonksiyonlar TokenStream alır ve döner.

    attributes'tan kastımız serialize isimli niteliğin aldığı parametrelerdir.
    input ise serialize niteliğinin uygulandığı tipin kod içeriğini taşır.
*/
#[proc_macro_attribute]
pub fn serialize(attributes: TokenStream, input: TokenStream) -> TokenStream {
    // Case 1: Olayı anlamaya çalışıyorken.

    // println!("Attributes Stream: \"{}\"", attributes.to_string());
    // println!("Item Stream: \"{}\"", input.to_string());
    // input

    // Case 2: Gelen syntax tree'yi yakalayıp identifier bilgisini almak ve farklı bir TokenStream (get_pi fonksiyonu) döndürmek.

    // // input'u Syntax Tree'ye çevirmek için syn modülündeki parse komutundan yararlanılıyor.
    // let syntaxtree: DeriveInput = syn::parse(input).unwrap();
    // let name = &syntaxtree.ident; // makronun uygulandığı tipin tanımlayıcısını(adını) ident ile almaktayız.
    // println!("Tipin adı {}", name);

    // // Burada quote! makrosundan yararlanarak TokenStream'i değiştirdik.
    // // Ve sonuç olarak serialize niteliğinin uygulandığı tip yerine derleyici get_pi fonksiyonunu dahil etti.
    // let expanded = quote! {
    //     fn get_pi() -> f32 {
    //         3.1415
    //     }
    // };
    // TokenStream::from(expanded)

    // Case 3: Makronun uygulandığı tipe yeni fonksiyon eklemek.
    let syntaxtree: DeriveInput = syn::parse(input).unwrap();
    let name = &syntaxtree.ident;

    // Çıktı kodunu ürettiğimiz yer
    let expanded = quote! {

        // Bir struct'a uygulayacağımızı düşündüğümden uygulandığı struct'a ait kod ağacını olduğu gibi korumak istedim.
        #syntaxtree

        // Struct'a bir fonksiyon ilavesi yaptığımız yer.
        impl #name{
            fn to_json(&self) -> String {
                // Sembolik bir JSON çıktısı
                String::from("{'result':'JSON World'}")
            }
        }

    };
    // Üretilen kod parçasını derleyiciye servis ettiğimiz yer
    TokenStream::from(expanded)
}

// Case 4: Custom Derive macro
// Bu sefer derive türünden bir makro kullanıyoruz.
// Bunun için tek yapmamız gereken fonksiyonu proc_macro_derive ile donatmak

#[proc_macro_derive(Memory)]
// Derive türünden makrolarda yine TokenStream girdisi ve çıktısı söz konusudur.
pub fn memory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Bu sefer girdi kodunu syn crate'içindeki parse_macro_input! ile yakalıyoruz.
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident; // Türün adını alıp

    // ona mem_usage isimli bir metot ekliyoruz.
    let expanded = quote! {
        impl #name {
            // Dummy bir fonksiyon. Güya yapının bellek kullanımını bulacak gibi düşünelim.
            fn mem_usage(&self) -> i32 {
                1024
           }
        }
    };

    TokenStream::from(expanded)
}
