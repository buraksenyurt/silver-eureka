use std::boxed::Box; // Box tipi heap bellek bölgesinde yer ayırıp işaret etmemizi sağlayan bir pointer'dır
use std::cmp; // sıralama ve karşılaştırma ile ilgili fonksiyonellikleri taşıyan Rust modülüdür
use std::ops::Index; // değişmez(immutable) içeriklerde indeksleme operasyonları için kullanılan Trait

const DEFAULT_SIZE: usize = 4; // listemizin varsayılan koşullardaki başlangıç kapasitesini belirttiğimiz sabit

type Item<T> = Option<T>; // generic Option türünden bir tip tanımladık. Bu listemizin tutacağı eleman dizisinin veri türü olacak.

// generic veri modelimiz
pub struct List<T>
where
    T: Sized + Clone, // T tipinin boyutlandırılabilir ve kopyalanabilir olması için Rust'ın varsayılan Sized ve Clone Trait'lerini uygulaması gerekir
{
    buffer: Box<[Item<T>]>, // Heap'te generic Item nesnelerinden oluşan bir diziyi referans eden field.
    capacity: usize,        // Kapasite bilgisi
    pub length: usize,      // Kaç eleman taşıdığımız bilgisi
}

// Generic listemiz için gerekli fonksiyon uyarlamaları.
// Burada da T tipi için Sized ve Clone Trait kıstaslarımız var.
impl<T> List<T>
where
    T: Sized + Clone,
{
    // Default Constructor olarak düşünebiliriz.
    pub fn new() -> List<T> {
        List {
            buffer: vec![None; DEFAULT_SIZE].into_boxed_slice(), // Başlangıçta DEFAULT_SIZE sabitinde belirttiğimiz kapasitede bir vektör tanımlıyoruz. Vector içeriğini generic box türüne çevirir.
            capacity: DEFAULT_SIZE,                              // İlk kapasitemiz belli
            length: 0,                                           // eleman sayımız tabii ki 0
        }
    }

    // yeni bir eleman eklemek için kullanılan generic fonksiyon
    pub fn add(&mut self, value: T) {
        if self.length == self.capacity {
            // eğer eleman sayısı kapasite ile anyı ise,
            self.increase(self.length + 1); // artıralım
        }
        self.buffer[self.length] = Some(value); // gelen veriyi sona ekleyelim
        self.length += 1; // eleman sayımızı bir artıralım
    }

    // Belli bir konumdaki elemanı elde etmek için kullanılan fonksiyon
    pub fn get(&mut self, index: usize) -> Item<T> {
        if self.length > index {
            // istenen pozisyon uygun bir aralıkta ise
            self.buffer[index].clone() // bir klonunu oluşturup geri ver
        } else {
            None // yoksa Option sabitinin None değerini döndür
        }
    }

    // Kapasitenin varsayılan olarak iki kart artırılmasını baz alana büyüme fonksiyonu
    fn increase(&mut self, min_capacity: usize) {
        let old_capacity = self.buffer.len(); // var olan kapasiteyi bir alalım
        let mut new_capacity = old_capacity + (old_capacity >> 1); // yeni kapasiteyi artıralım

        new_capacity = cmp::max(new_capacity, min_capacity);
        new_capacity = cmp::min(new_capacity, usize::max_value());
        let current = self.buffer.clone(); // güncel eleman dizisini koplayıyoruz
        self.capacity = new_capacity; // Yeni kapasite atanıyor

        self.buffer = vec![None; new_capacity].into_boxed_slice(); // Yeni elemanları vector dizisini box dizisine çevirerek atıyoruz.
        self.buffer[..current.len()].clone_from_slice(&current);
    }
}
