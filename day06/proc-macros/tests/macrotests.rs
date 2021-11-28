use proc_macros::*;

#[serialize(json, 2)]
struct Player {
    id: i32,
    name: String,
    level: u8,
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

    // Case 2: Bu oldukça enteresan oldu.
    // Ortamda get_pi diye bir fonksiyon yok. Ancak serializable makrosu içinden quote! makrosunu kullanarak derleme aşamasında
    // bu fonksiyonun koda eklenmesini sağlıyoruz.
    let pi = get_pi();
    assert_eq!(pi, 3.1415);
}
