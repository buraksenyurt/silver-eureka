#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_fn_works_test() {
        let mut numbers = vec![3, 5, 6, 15, 2, 4, 8, 10];
        assert_eq!(numbers.len(), 8);
        assert_eq!(split(&mut numbers, 3), &[&[3, 5], &[6, 15], &[2, 4]]); // 8 / 3 ~ 2 olarak ele alınacağında ikişerli elemanlar halinde bölünme söz konusudur
        assert_eq!(split(&mut numbers, 5), &[&[3], &[5]]); // 8/5 ~ 1 olarak ele alınacağından birer parça halinde bölünme söz konusu
    }

    #[test]
    fn should_split_fn_fails_test() {
        let mut numbers = vec![3, 5, 6, 15, 2, 4, 8, 10];
        assert_eq!(split(&mut numbers, 2), &[&[3, 5, 6, 15], &[2, 4, 8, 10]]); // Burası ilginçtir. 8/2 = 4 parçalı elemanların dönüşünü bekliyoruz. Lakin bu şekilde 8 elemanı da ele almış olacağız. 
    }
}

#[warn(dead_code)]
use std::slice;

// split fonksiyonu parametre olarak gelen slice'ı belirtilen sayıya göre eşit parçalar halinde ayırmak üzere kullanılmaktadır.

pub fn split<T>(data: &mut [T], parts: usize) -> Vec<&mut [T]> {
    let length = data.len(); // gelen slice'ın uzunluğunu alıyoruz.
    let step = length / parts; // işlemler için adım sayısını buluyoruz
    unsafe {
        // derleyicinin güvenli kod mekanizmasını kapattığımız bölüm
        let pointer = data.as_mut_ptr(); // as_mut_ptr fonksiyonu, slice'ın pointer referansını, bellek sınır kontrolü yapmadan döndürür.
        (0..step + 1)
            .map(|i| {
                let offset = (i * step) as isize;
                let start = pointer.offset(offset);
                slice::from_raw_parts_mut(start, step) // ilk parametre işaretçiyi götüreceğimiz bellek adresi iken ikinci parametre buradan itibaren ne kadar uzunlukta bir alanı keseceğimizi belirtiyor.
            })
            .collect()
    }
}
