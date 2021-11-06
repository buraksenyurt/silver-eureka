#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_regular_works_test() {
        let result = sum_regular(&[3, 4, 7, 9, 16]);
        assert_eq!(result, 411);
    }

    #[test]
    fn should_sum_parallel_works_test() {
        let result = sum_parallel(&[3, 4, 7, 9, 16]);
        assert_eq!(result, 411);
    }

    #[test]
    fn should_regular_quick_sort_works_test() {
        let mut numbers = vec![32, 23, 8, 5, 9, 1, 10, 90, 3, 2];
        qsort_regular(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 5, 8, 9, 10, 23, 32, 90]);
    }
}
// veriyi paralel işleme üzerine başarılı rust crate'lerinden birisi olan rayon kullanılmakta
use rayon::prelude::*;

// standart bir toplama fonksiyonu
// gelen slice içindeki elemanların karelerini alıp topluyor
pub fn sum_regular(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

pub fn sum_parallel(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum() // burada ise rayon ile gelen par_iter fonksiyonu çağırılmakta. Bu iterasyonun paralel ele alınması için kullanılıyor.
}

// İzleyen fonksiyonlar quicksort sıralaması için kullanılmakta.
pub fn qsort_regular(v: &mut [i32]) {
    if v.len() > 1 {
        let middle = partition(v);
        let (low, high) = v.split_at_mut(middle);
        qsort_regular(low);
        qsort_regular(high);
    }
}

pub fn partition(v: &mut [i32]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
