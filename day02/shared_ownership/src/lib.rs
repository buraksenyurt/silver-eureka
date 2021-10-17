#[cfg(test)]
mod tests {
    use super::*;

    #[test] // #3
    fn loop_test() {
        let word = "rustician".to_owned();
        for _ in 0..100 {
            assert_eq!(get_length(word.clone()), 9);
        }
    }

    #[test] // #2
    fn fix_out_of_scope_test() {
        // out_of_scope testindeki hatayı çözmek için clone metodundan yararlanılır
        // Bu durumda word değişkeni bu scope'ta yaşamaya devam edebilir
        let word = "rustician".to_owned();
        assert_eq!(get_length(word.clone()), 9);
        assert_eq!(word.len(), 9);
    }

    #[test] // #1
    fn out_of_scope_test() {
        let word = "rustician".to_owned();
        assert_eq!(get_length(word), 9); // Dikkat!
                                         // assert_eq!(word.len(), 9); // word değişkeni get_length fonksiyonuna aktarıldıktan sonra scope dışında kalır ve bu nedenle bu satırda derleyici hata verir. Açıp hatayı görün
    }
}

pub fn get_length(something: String) -> usize {
    something.len()
}
