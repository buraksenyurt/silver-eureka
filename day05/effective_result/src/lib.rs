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
        assert_eq!(find_suspicies(query, ""), 0); // teorik olarak burada da -1 döndürmesini bekleriz ama 0 döner. Esasında mutlaka bir arama metni olmasına zorlamamız gerekir.
        assert_eq!(find_suspicies("", "delete from"), -1); // Burada da bir sorun var. delete from ifadesini arayacağım metin yok. Bu durum aslında bir hata(Error) olarak yorumlanmalı.
    }

    #[test]
    fn little_better_practice_test() {
        let query = "Select * from players;1=1";
        let wanted = "&";
        let found = find_suspicies2(query, wanted);
        // Bu kez Option<usize> dönen bir fonksiyon söz konusu.
        // Bir değer bulunduysa Some, bulunmadıysa None ile sonuçları değerlendirebiliriz.
        assert_eq!(found, None);
        assert_eq!(find_suspicies2(query, ";1=1"), Some(21));
        assert_eq!(find_suspicies2(query, ""), Some(0)); // Aslında burada da None dönmesini isteriz ama Some(0) dönmüştür. Dolayısıyla hata yönetimini daha iyi yapmamız gerekmektedir.
        assert_eq!(find_suspicies2("", "delete from"), None); // Sorun devam ediyor. delete from ifadesini arayacağım metin yine yok. Bu durum aslında bir hata(Error) olarak yorumlanmalı.
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

/*
    Biraz Daha İyisi:

    Daha iyi bir yaklaşım -1 yerine Option enum'ını döndürmektir.
    Option enum'ına göre fonksiyon ya bulunan değeri döner ya da None.
*/
pub fn find_suspicies2(content: &str, wanted: &str) -> Option<usize> {
    content.find(wanted) // find metodu otomatik olarak Option döndürür.
}

/*
    En iyi pratik:


*/
