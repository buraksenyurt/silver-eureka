extern crate proc_macro;
use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

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

    // input'u Syntax Tree'ye çevirmek için syn modülündeki parse komutundan yararlanılıyor.
    let syntaxtree: DeriveInput = syn::parse(input).unwrap();
    let name = &syntaxtree.ident; // makronun uygulandığı tipin tanımlayıcısını(adını) ident ile almaktayız.
    println!("Tipin adı {}", name);

    // Burada quote! makrosundan yararlanarak TokenStream'i değiştirdik.
    let expanded = quote! {
        fn get_pi() -> f32 {
            3.1415
        }
    };
    TokenStream::from(expanded)
}
