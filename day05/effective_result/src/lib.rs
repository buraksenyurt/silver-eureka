#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_practice_test() {
        let query = "Select * from players;1=1";
        let wanted = "&";
        let found = find_suspicies(query, wanted);
        assert_eq!(found, -1);
        assert_eq!(find_suspicies(query, ";1=1"), 21);
        assert_eq!(find_suspicies(query, ""), 0);
    }
}

/*
    Kötü Pratik:

    İlk önce aşağıdaki fonksiyonu göz önüne alalım.
    Belli bir metin içinde parametre olarak verilen bilginin başladığı konumu döndürür.
    Bulamazsa -1 döndürdüğümüzü ve bunun error olarak çağıran tarafça yorumlanması gerektiğini düşünelim.
*/
pub fn find_suspicies(content: &str, wanted: &str) -> i32 {
    content.find(wanted).map(|c| c as i32).unwrap_or(-1)
}
