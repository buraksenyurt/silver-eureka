#[warn(dead_code)]

fn main() {
    /*
     İlk olarak scope dışına çıkıldığına meydana gelen drop işlemine bakmak lazım.
     Bir değişkene değer atadığımızda başlayan sahiplik scope sonlanınca biter ve değişkenin tuttuğu değerlerde bellekten düşürülür.
     Bunun işleyişini görmek için Drop trait'i ile belirtilen davranışı kullandığımız örneğe bakalım.

     Scope içinde tanımlanan article isimli değişken scope dışına çıktığında drop trait'i de çalışır.
     Bu kaynağın bellekten düşürüldüğünün bir nevi ispatıdır.
     (.Net tarafındaki Dispose gibi lakin Rust ortamında bir Garbage Collector mekanizması olmadığını hatırlayalım)
    */
    println!("Scope'a girilmeden önce.");
    {
        println!("\tScope'a girildi");
        let article = Article {
            title: String::from("Amatörler için Ownership Konusu"),
            id: 1,
        };
        println!("\t({}) - {}", article.id, article.title);
        println!("\tScope'tan çıkılmak üzere.");
    }

    println!("Scope'tan çıkıldı.");
}

// Kobay struct
struct Article {
    title: String,
    id: i32,
}

// Drop trait'ini implemente ettik
impl Drop for Article {
    fn drop(&mut self) {
        println!("Article için drop işlemi.")
    }
}
