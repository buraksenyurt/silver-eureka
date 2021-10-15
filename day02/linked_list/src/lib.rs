#[cfg(test)]
mod tests {}

/*
Basit bir LIFO veri yapısı oluşturacağız. Liste yapısının her bir elemanı bir boğumu ifade edecek.
Node yapısı bir değer ve kendinden sonraki Node'un referansını taşır. Sonraki referansın taşınabilmesi için Rc<RefCell> pointer kullanılır.
RefCell -> Dinamik olarak kontrol edilen ödünç alma(borrow) kurallarına sahip değiştirilebilir(mutable) bir bellek konumu tahsis eder.
Rc -> Referance Counting Pointer. Rc<T> ile heap içinde tahsis edilen T türünde bir değerin sahipliğinin paylaşımı sağlanır. Özellikle Rc üzerinden clone fonskiyonu çağrısı yaparak, heap üstünde aynı tahsise sahip yeni bir işaretçi üretilebilir
Rc<RefCell<Node>> kullanımı Node'un bellekten çekilmesi ve değiştirilebilmesinde yardımcı olur.
Ancak Node kullanılan her yerde Rc<RefCell<Node>> yazmak yerine örnekte Alias kullanımı tercih edilmiştir.
*/

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

// Linked List içindeki her bir elemanı temsil eder
#[derive(Clone)] // Node klonlanabilir
struct Node {
    info: String,
    next: Link, // Sonraki boğum referansı
}

// Linked List veri yapısı
struct ProcessLog {
    first: Link, // İlk boğum referansı
    last: Link, // Son boğum referansı
    pub count: u64, // Toplam eleman sayısı
}

impl Node {
    // Yeni bir Node nesnesi örneklemek için kullanılır.
    // Geriye pointer döner
    fn new(info: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            info: info,
            next: None,
        }))
    }
}
