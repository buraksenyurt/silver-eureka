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
        assert_eq!(split(&mut numbers, 2), &[&[3, 5, 6, 15], &[2, 4, 8, 10]]); // Burası ilginçtir. 8/2 = 4 parçalı elemanların dönüşünü bekliyoruz. Lakin bu şekilde 8 elemanı da ele almış olacağız. Dolayısıyla pek beklemediğimiz değerlerin çekildiğini göreceğiz.
    }

    #[test]
    fn reading_memory_test() {
        let bytes = unsafe {
            std::mem::transmute::<&str, &[u8]>("Rust zevkli ama öğrenilmesi zor bir dil sanki")
        };
        assert_eq!(
            bytes,
            &[
                82, 117, 115, 116, 32, 122, 101, 118, 107, 108, 105, 32, 97, 109, 97, 32, 195, 182,
                196, 159, 114, 101, 110, 105, 108, 109, 101, 115, 105, 32, 122, 111, 114, 32, 98,
                105, 114, 32, 100, 105, 108, 32, 115, 97, 110, 107, 105
            ]
        );
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
