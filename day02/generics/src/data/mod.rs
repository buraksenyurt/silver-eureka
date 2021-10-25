use std::boxed::Box; // Box tipi heap bellek bölgesinde yer ayırıp işaret etmemizi sağlayan bir pointer'dır
use std::cmp; // sıralama ve karşılaştırma ile ilgili fonksiyonellikleri taşıyan Rust modülüdür
use std::ops::Index; // değişmez(immutable) içeriklerde indeksleme operasyonları için kullanılan Trait

const DEFAULT_SIZE: usize = 4; // listemizin varsayılan koşullardaki başlangıç kapasitesini belirttiğimiz sabit

type Item<T> = Option<T>;

// generic veri modelimiz
pub struct List<T>
where
    T: Sized + Clone, // T tipinin boyutlandırılabilir ve kopyalanabilir olması için Rust'ın varsayılan Sized ve Clone Trait'lerini uygulaması gerekir
{
    buffer: Box<[Item<T>]>,
    capacity: usize,
    pub length: usize,
}
