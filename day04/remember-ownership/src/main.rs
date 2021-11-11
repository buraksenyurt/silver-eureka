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

    //case_4_move();

    /*
        #5

        built-in olan ve maliyeti düşük veri türleri Copy, Clone trait'lerini uyguladıklarından move operasyonları sonrası borrow ihlallerine takılmazlar.
        struct, kendi veri yapılarımızı tanımlamak için kullanılır ve varsayılan olarak Copy trait'leri uygulanmadığında move operasyonlarından sonra kullanılmaz hale gelirler.
    */
    // case_5_copy();

    /*
        #6

        5nci çözümde copy,clone trait'leri ile move operasyonlarında borrow ihlallerine takılmamıştık.
        Bir başka alternatif yol & operatörü ile değişkenin ödünç verilmesidir.
    */
    // case_6_borrow_with_ampersand();

    /*
        #7

        & operatörünü metot parametlerinde de kullanabiliriz.
    */
    case_7_ampersand_parameter();

    /*

        #8

        Metotlara yapılan parametre aktarımlarını özetleyelim.

        - Eğer değişken Copy trait'ini uygulamış ve ödünç alınmamışsa(borrow) ilgili metoda değer olarak taşınır.
        - Eğer değişken Copy trait'ini uygulamış ve birisi tarafından ödünç alınmışsa ilgili metoda referans olarak taşınır.
        - Eğer değişken Copy trait'ini uygulamamışsa mutlak suretle ödünç alınmalıdır(& ile bunu sağlamıştık). Ancak bu şekilde referans olarak metoda taşınabilir.

        Aşağıdaki metot bu kuralları özetlemek üzere yazılmıştır.

    */
    case_8_basic_rules();
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

fn case_5_copy() {
    let location1 = Point {
        x: 1.12,
        y: 1.25,
        z: 1.23,
    };
    // Aşağıdaki yeniden atama (reassignment) sonrası location2 kullanılamaz.
    // Ama Copy, Clone trait'lerini uygulamadığı için.
    let location2 = location1;
    println!("{}\n{}", location1.x, location2.x);
}

fn case_6_borrow_with_ampersand() {
    let black_hole = Location { high: 100, low: 10 };
    // let rabbit_hole = black_hole; // bu kullanım yine ödünç alma ihlallerine takılacaktır.
    let rabbit_hole = &black_hole; // Bu kullanımda ise
    println!("{}\n{}", black_hole.high, rabbit_hole.high);
}

// dikkat edileceği üzere points vektörü ilgili metoda & operatörü ile aktarılır. Yani referansı yollanır.
// Bu sebeple ödünç alma ihlali oluşmaz.
// Hatırlanacağı üzere vec türü dinamik boyutlu bir enstrüman olduğundan ve bellekte ne kadar yer tutacağı bilinmeyeceğinden Copy trait'ini uygulamaz.
// Ancak & ile referansını taşıma şansımız söz konusudur.
fn case_7_ampersand_parameter() {
    let points = vec![1, 4, 8, 12];
    let result = sum_of_square2(&points);
    println!("{:?}\nSayıların kareleri toplamı {}", points, result);
}

fn case_8_basic_rules() {
    // 32 sayısı i32 türünden kabul edilecektir ve bu tür Copy trait'ini uygulamaktadır.
    let some_number = 32;
    // Aşağıdaki kullanımda move operasyonu söz konusudur. Değer kopyalanarak taşınır ve some_number tekrar kullanılamaz.
    let is_positive_by_value = send_by_value(some_number);
    // Bu kullanımda ise referans olarak taşınması söz konusudur. Dolayısıyla yeniden kullanılabilir
    let is_positive_by_ref = send_by_reference(&some_number);

    // Vektörler bilindiği üzere Copy davranışı sergilemezler
    let numbers = vec![];
    // Doğal olarak metoda referans olarak taşınırlar
    let is_empty = send_vec_by_reference(&numbers);

    println!("{}", is_positive_by_value);
    println!("{}", is_positive_by_ref);
    println!("Vektör Sayıları {:?} is_empty: {}", numbers, is_empty);
}

fn send_by_reference(number: &i8) -> bool {
    number.is_positive()
}

fn send_by_value(number: i8) -> bool {
    number.is_positive()
}

fn send_vec_by_reference(numbers: &Vec<i8>) -> bool {
    numbers.is_empty()
}

struct Location {
    high: i32,
    low: i32,
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

// izleyen fonskiyon bir üsttekinden farklıdır.
// parametrede & operatörü kullanılmıştır.
// Yani gelen vektörün referansı içeriye alınmaktadır.
fn sum_of_square2(numbers: &Vec<i32>) -> i32 {
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

// Copy ve Clone davranışlarını varsayılan halleri ile uygulattık
// Bu durumda move operasyonlarında değerler kopyalanarak taşınacaktır.
// struct'lar stack bellek bölgesinde yaşadıklarından maliyet açısından masraflı değillerdir.
// Yine de varsayılan move kurallarına tabilerdir. Biz trait uygulayarak aksini söylemedikçe.
// Elbette aşağıdaki yapıda dikkat çeken bir nokta alanların da Copy trait uygulanmış built-in tipler olmasıdır.
// Aynı şeyi Article Struct'ı için denersek farklı bir sorunla karşılaşırız.
// String, ne kadar yer kapladığı bilenemeyecek bir tür olduğundan Rust bu işe sıcak bakmayacak ve Copy trait'ini uygulatmayacaktır.
#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

// Drop trait'ini implemente ettik
impl Drop for Article {
    fn drop(&mut self) {
        println!("Article için drop işlemi.")
    }
}
