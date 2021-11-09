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

    //case_1_drop();

    /*
        #2

        Diğer mesele move olayı. Rust'a göre bir değerin sadece tek bir sahibi olabilir.
        Özellikle değişken atamalarında sahiplik yeni değişkene geçer ve diğer kullanılmaz hale gelir.
    */

    //case_2_move();

    /*
        #3

        Move durumunun zor fark edildiği noktalardan birisi fonksiyon parametrelerinde kullanılmalarıdır.
        İlgili örnekte bir Vector dizisinin bir fonksiyona parametre olarak geçilmesi söz konusudur.
        Burada parametre bazında bir move işlemi söz konusudur ve sahiplik diğer fonksiyon içine taşınır.
        Ve beklediğimiz hata oluşur.

        Fakat;

        #4ncü örneğe dikkat
    */

    //case_3_move();

    /*
        #4

        İşlerin ilginçleşeceği nokta bu örnektir.
        3ncü case'deki örnekte vektörü fonksiyona parametre olarak taşıdıktan sonra kullanamadık.
        Oysaki i32 türünden değişkenleri gönderdiğimizde bir sorun olmadan fonksiyon çağrısı sonrası da kullanabildik.
        Bunun sebebi i32 türünün diğer birçok bellek maliyeti düşük veri türü gibi Copy davranışını(trait) uygulamış olmasıdır.
        Vectör türünde bu trait uygulanmadığından move operasyonu farklı şekilde yorumlanır ve borrow rules işletilir.
        Çok doğal olarak bu ayrım kendi veri yapılarımız(struct) için de geçerlidir -> #5nci örnek.
    */

    case_4_move();
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

// fn case_2_move() {
//     // Aşağıda bir vector dizisi oluşturuluyor. Vektör türleri dinamik büyüyebilen dizilerdir.
//     let points: Vec<i32> = [60, 55, 100, 90].to_vec();
//     // Burada klasik bir yeniden atama (reassignment) işlemi söz konusu
//     // Ve sahiplik bu noktada new_points'e taşınır (move)
//     let new_points = points;

//     // Sahipliğin taşınması ve bir değere sadece bir değişken sahip olabilir ilkesi nedeniyle aşağıdaki kod derleme zamanı hatası verecektir.
//     // value borrowed here after move
//     println!("Points: {:?}\nNew Points: {:?}", points, new_points);
// }

// fn case_3_move() {
//     let points = vec![1, 4, 8, 12];
//     let result = sum_of_square(points);
//     // Vektör sum fonksiyonuna parametre olarak geçtiğinde sahipliği ilgili fonksiyona taşınmış olur.
//     // Bu nedenle aşağıdaki kullanımda yine "value borrowed here after move" hatası alınacaktır
//     println!("{:?}\nSayıların kareleri toplamı {}", points, result);
// }

fn case_4_move() {
    let first_value = 10;
    let second_value = 20;
    let total = sum_of_two(first_value, second_value);
    println!("{} + {} = {}", first_value, second_value, total);
}

// Kobay bir fonksiyon.
// parametre olarak gelen x ve y değerlerini toplar
fn sum_of_two(x: i32, y: i32) -> i32 {
    x + y
}

// izleyen fonksiyonu parametre olarak gelen vektördeki sayıların kareleri toplamını bulur
fn sum_of_square(numbers: Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n * n
    }
    sum
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
