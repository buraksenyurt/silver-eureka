#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_scope() {
        let word = "rustician".to_owned();
        assert_eq!(get_length(word), 9);
        // assert_eq!(word.len(), 9); // word değişkeni get_length fonksiyonuna aktarıldıktan sonra scope dışında kalır ve bu nedenle bu satırda derleyici hata verir. Açıp hatayı görün
    }
}

pub fn get_length(something: String) -> usize {
    something.len()
}
