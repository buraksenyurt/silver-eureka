mod list;

use list::*;

/*
    Iterator kullanımında üzerinden dolaşılacak asıl veri yapısını referans eden veri yapılarına başvurulur.
    Örneğin bu programdaki List<T> elemanları için ListIterator<T> veri yapısı tasarlanmıştır.
    Liste üzerinde hareket etme kabiliyeti için Iterator ve IntoIterator isimli trait'lerin de implemente edilmesi gerekir.

    Iterator isimli trait'in birçok metodu vardı ancak mutlak uygulanması gereken fonksiyonu next'tir.
    Next fonksiyonu iterasyonun ilerletilmesini ve bir sonraki değerin döndürülmesini sağlar. Örnekte bunun için pop metotu kullanılmaktadır.
    pop sayesinden ilk elemandan itibaren son elemana kadar hareket edilebilir.

    Bir for döngüsünün bu iterasyondan yararlanabilmesi ve List<T> elemanlarında kullanılabilmesi için IntoIterator isimli trait'in uygulanması gerekir.
*/
pub struct ListIterator<T>
where
    T: Sized + Clone,
{
    list: List<T>, // referans edilen List<T>
}

impl<T> ListIterator<T>
where
    T: Sized + Clone,
{
    // Yeni bir Iterator nesnesinin örneklenmesinde kullanılır
    fn new(list: List<T>) -> ListIterator<T> {
        ListIterator { list: list }
    }
}

impl<T> Iterator for ListIterator<T>
where
    T: Sized + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.list.pop()
    }
}

impl<T> IntoIterator for List<T>
where
    T: Sized + Clone,
{
    type Item = T;
    type IntoIter = ListIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_iterator_works_for_list_test() {
        let mut points: List<i32> = List::new();
        points.append(3);
        points.append(6);
        points.append(1);
        points.append(9);
        assert_eq!(points.count, 4);

        // iterasyon nesnesi oluşturulur
        let mut iter = points.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_for_loops_works_for_list_test() {
        let mut points: List<i32> = List::new();
        points.append(3);
        points.append(6);
        points.append(1);
        points.append(9);
        let mut total: i32 = 0;
        for p in points {
            total += p;
        }
        assert_eq!(total, 19);
    }
}
