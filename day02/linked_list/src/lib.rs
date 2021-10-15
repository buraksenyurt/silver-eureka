#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_new_processlog_is_empty_test() {
        let mut logs: ProcessLog = ProcessLog::new();
        assert_eq!(logs.count, 0);
        assert_eq!(logs.pop(), None);
    }

    #[test]
    fn should_append_and_pop_works_test() {
        let mut logs: ProcessLog = ProcessLog::new();
        logs.append("Job started".to_owned());
        logs.append("Data collected".to_owned());
        logs.append("Prepearing...".to_owned());
        logs.append("Reports created".to_owned());
        logs.append("Job done".to_owned());
        assert_eq!(logs.count, 5);
        assert_eq!(logs.pop(), Some("Job started".to_owned()));
        assert_eq!(logs.count, 4);
        let log = logs.pop();
        assert_eq!(log, Some("Data collected".to_owned()));
        assert_eq!(logs.count, 3);
        logs.pop();
        logs.pop();
        logs.pop();
        assert_eq!(logs.count, 0);
    }
}

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
    pub fn append(&mut self, info: String) {
        let new_node = Node::new(info); // gelen veri ile yeni bir Node nesnesi örneklenir

        match self.last.take() {
            // pattern matchin ile liste sonundaki Node örneğine bakılır
            Some(there) => there.borrow_mut().next = Some(new_node.clone()), // Eğer sonda bir eleman varsa onun sonraki referans adresine new_node'un bir klonu eklenir
            None => self.first = Some(new_node.clone()), // Sonda bir elaman yoksa liste boş anlamına gelir ve new_node'un klonu ilk eleman olarak yerleştirilir
        };

        self.count += 1; // eleman sayısı bir artırırlır
        self.last = Some(new_node); // Match ne olursa olsun son eleman yeni üretilen eleman olacaktır
    }

    // İlk eklenen elemanı listeden çekmek(aynı zamanda kaldırmak) için kullanılır. İlk eleman çıkarılınca yeni ilk elaman sonraki olarak değişmeli ve diğer referans bağlantıları da buna göre güncellenmelidir
    pub fn pop(&mut self) -> Option<String> {
        self.first.take().map(|item| {
            // ilk elemandan itibaren map çağrısı ile devam eden Node örneklerine bakılan bir iterasyon başlatılır.
            if let Some(next) = item.borrow_mut().next.take() {
                // Sonraki adreste bir Node var mı?
                self.first = Some(next); // O zaman onu başa yerleştir
            } else {
                // Artık arkadan gelen bir Node yok
                self.last.take(); // O zaman şu anki item, veri yapısının son elemanı olarak kabul edilir
            }

            self.count -= 1; // Eleman sayısı bir azaltılır

            // İlk girilen eleman değeri dönülür
            Rc::try_unwrap(item)
                .ok()
                .expect("Bir sorun gördüm sanki. Gördüm, gördüm. Bi sorun gördüm.")
                .into_inner()
                .info
        })
    }
}
