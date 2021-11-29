use proc_macros::*;

//#[serialize(json, 2)]
#[serialize]
struct Player {
    id: i32,
    name: String,
    level: u8,
}

// Case 4 : Derive Macro
#[derive(Memory)]
struct Product {
    id: i32,
    title: String,
    price: f32,
}

#[test]
fn should_serialize_works_test() {
    // Case 1: Player struct'ı sorunsuz şekilde oluşur ve ilk compile sırasında serialize makrosundaki case 1 çalışır.
    // Yani argüman ve struct'a ait Syntax Tree terminale basılır.
    // let _marine = Player {
    //     id: 2,
    //     name: "Marine Corp".to_owned(),
    //     level: 10,
    // };

    // Case 2: Bu oldukça enteresandı. Player struct'ı yerine get_pi fonksiyonunu elde ettik.
    // Aslında ortamda get_pi diye bir fonksiyon yok. Ancak serializable makrosu içinden quote! makrosunu kullanarak derleme aşamasında
    // bu fonksiyonun koda eklenmesini sağlıyoruz.
    // let pi = get_pi();
    // assert_eq!(pi, 3.1415);

    // Case 3: Bir struct türüne otomatik olarak to_json fonksiyonunun derleme zamanında eklenmesi durumu.
    let marine = Player {
        id: 11,
        name: "Martinez Hose De La Cruz Dos Santos Amigos".to_owned(),
        level: 99,
    };
    // to_json metodu normalde Player isimli struct için yazılmış değil.
    // serialize makrosu ile derleme aşamasında eklenen bir fonksiyon.
    let jsoned = marine.to_json();
    assert_eq!(jsoned, "{'result':'JSON World'}");
}

// Case 4: Dervie makrosunun test kodu
#[test]
fn should_derive_macro_works() {
    let monitor = Product {
        id: 1,
        title: "Filips 38.1234 inch Cörv Ultra Süper Eyç Di Monitör".to_owned(),
        price: 1500.99,
    };
    let result = monitor.mem_usage(); // mem_usage fonksiyonu Memory isimli makro ile eklenmiştir.
    assert_eq!(result, 1024);
}
