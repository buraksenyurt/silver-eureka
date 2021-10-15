use std::cell::RefCell;
use std::rc::Rc;

// Node veri yapısındanki sonraki eleman referansı, List<T> veri yapısındaki ilk ve son eleman referanslarını kolayca yazabilmek için kullanılan alias ifadesi
type Item<T> = Option<Rc<RefCell<Node<T>>>>;

// T türünden Node veri yapısı.
#[derive(Clone)]
struct Node<T>
where
    T: Sized + Clone, // Generic kıstas. T boyutu derleyici tarafında bilinebilen ve klonlanabilir bir tür olmalı
{
    value: T,      // T tipinden veri
    next: Item<T>, // Sonraki T tipinden Node nesne referansı
}

impl<T> Node<T>
where
    T: Sized + Clone,
{
    // Yeni bir Node nesnesinin oluşturulmasını sağlar
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

// List isimli generic veri türümüz
#[derive(Clone)]
pub struct List<T>
where
    T: Sized + Clone,
{
    first: Item<T>,   // İlk Node elemanının referansı
    last: Item<T>,    // Son Node elemanının referansı
    pub count: usize, // Eleman sayısı
}

impl<T> List<T>
where
    T: Sized + Clone,
{
    // Yeni bir generic List nesnesi oluşturmak için
    pub fn new() -> List<T> {
        List {
            first: None,
            last: None,
            count: 0,
        }
    }

    // Listeye T türünden eleman eklemek için
    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.last.take() {
            Some(there) => there.borrow_mut().next = Some(new_node.clone()),
            None => self.first = Some(new_node.clone()),
        };
        self.count += 1;
        self.last = Some(new_node);
    }

    // İlk eklenen elemanı listeden çekip almak için (Liste boyutunu da kısaltır ve Node'ları tekrardan yerleştirir)
    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|item| {
            if let Some(next) = item.borrow_mut().next.take() {
                self.first = Some(next);
            } else {
                self.last.take();
            }
            self.count -= 1;
            Rc::try_unwrap(item)
                .ok()
                .expect("Bi kedi gördüm san...Bi hata gördüm sanki.")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_new_list_iş_empty_test() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.count, 0);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn should_append_works_test() {
        let mut list = List::new();
        list.append(1);
        list.append(3);
        list.append(5);
        list.append(7);
        list.append(9);
        assert_eq!(list.count, 5);
    }

    #[test]
    fn should_pop_works_test() {
        let mut list = List::new();
        list.append(23);
        list.append(32);
        list.append(8);
        list.append(45);
        assert_eq!(list.count, 4);
        assert_eq!(list.pop(), Some(23));
    }
}
