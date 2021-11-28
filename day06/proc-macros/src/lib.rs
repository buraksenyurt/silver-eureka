extern crate proc_macro;
use proc_macro::TokenStream;

/*
    Attribute tabanlı makrolarda proc_macro_attribute niteliği ile imzalanan fonksiyonlar söz konusudur.
    Bu fonksiyonlar TokenStream alır ve döner.

    attributes'tan kastımız serialize isimli niteliğin aldığı parametrelerdir.
    input ise serialize niteliğinin uygulandığı tipin kod içeriğini taşır.
*/
#[proc_macro_attribute]
pub fn serialize(attributes: TokenStream, input: TokenStream) -> TokenStream {
    println!("Attributes Stream: \"{}\"", attributes.to_string());
    println!("Item Stream: \"{}\"", input.to_string());
    input
}
