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
    first: Link,    // İlk boğum referansı
    last: Link,     // Son boğum referansı
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

impl ProcessLog {
    // Yeni bir ProcessLog nesnesi üretir
    pub fn new() -> ProcessLog {
        ProcessLog {
            first: None, // Option enum sabiti gereği başlangıçta None olarak set edilir
            last: None,
            count: 0,
        }
    }

    // Listeye yeni bir Node eklemek için kullanılır
    pub fn push(&mut self, info: String) {
        let new_node = Node::new(info); // gelen veri ile yeni bir Node nesnesi örneklenir

        match self.last.take() {
            // pattern matchin ile liste sonundaki Node örneğine bakılır
            Some(there) => there.borrow_mut().next = Some(new_node.clone()), // Eğer sonda bir eleman varsa onun sonraki referans adresine new_node'un bir klonu eklenir
            None => self.first = Some(new_node.clone()), // Sonda bir elaman yoksa liste boş anlamına gelir ve new_node'un klonu ilk eleman olarak yerleştirilir
        };

        self.count += 1; // eleman sayısı bir artırırlır
        self.last = Some(new_node); // Match ne olursa olsun son eleman yeni üretilen eleman olacaktır
    }

    // Son eklenen elemanı listeden çekmek(aynı zamanda kaldırmak) için kullanılır
    pub fn pop(&mut self) -> Option<String> {
        self.first.take().map(|item| {
            // ilk elemandan itibaren map çağrısı ile devam eden Node örneklerine bakılan bir iterasyon başlatılır. Bu iterasyon sondaki elemanı hariç tutarak ProcessLog'un first ve last konumlandırmalarını değiştirmek üzere işletilir. Tipik Pop işlevi.
            if let Some(next) = item.borrow_mut().next.take() { // Eğer takip eden Node değeri varsa
                self.first = Some(next); // Onu first değişkenine alır. İterasyon son elemana gelene kadar first değişmeye devam eder
            } else { // Artık arkadan gelen bir Node kalmamışsa
                self.last.take();  // ProcessLog'un son elemanı olarak kabul edilir
            }

            self.count -= 1; // Eleman sayısı bir azaltılır

            // iterasyonun son elemanının değeri alınır 
            Rc::try_unwrap(item) 
                .ok()
                .expect("Bir sorun gördüm sanki. Gördüm, gördüm. Bi sorun gördüm.")
                .into_inner()
                .info
        })
    }
}
