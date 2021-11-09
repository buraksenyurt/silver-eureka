#[warn(dead_code)]

fn main() {
    /*
     #1

     İlk olarak scope dışına çıkıldığına meydana gelen drop işlemine bakmak lazım.
     Bir değişkene değer atadığımızda başlayan sahiplik scope sonlanınca biter ve değişkenin tuttuğu değerlerde bellekten düşürülür.
     Bunun işleyişini görmek için Drop trait'i ile belirtilen davranışı kullandığımız örneğe bakalım.

     Scope içinde tanımlanan article isimli değişken scope dışına çıktığında drop trait'i de çalışır.
     Bu kaynağın bellekten düşürüldüğünün bir nevi ispatıdır.
     (.Net tarafındaki Dispose gibi lakin Rust ortamında bir Garbage Collector mekanizması olmadığını hatırlayalım)
    */

    case_1_drop();

    /*
     #2

     Diğer mesele move olayı. Rust'a göre bir değerin sadece tek bir sahibi olabilir.
     Özellikle değişken atamalarında sahiplik yeni değişkene geçer ve diğer kullanılmaz hale gelir.
    */

    case_2_move();
}

fn case_1_drop() {
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

fn case_2_move() {
    // Aşağıda bir vector dizisi oluşturuluyor. Vektör türleri dinamik büyüyebilen dizilerdir.
    let points: Vec<i32> = [60, 55, 100, 90].to_vec();
    // Burada klasik bir yeniden atama (reassignment) işlemi söz konusu
    // Ve sahiplik bu noktada new_points'e taşınır (move)
    let new_points = points; 

    // Sahipliğin taşınması ve bir değere sadece bir değişken sahip olabilir ilkesi nedeniyle aşağıdaki kod derleme zamanı hatası verecektir.
    // 
    println!("Points: {:?}\nNew Points: {:?}", points, new_points);
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
