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

    #[test]
    fn best_practice_test() {
        let query = "Select * from players;1=1";
        let wanted = "&";
        let found = find_suspicies3(query, wanted);

        // Artık fonksiyon çağrısında olası hata durumlarını daha iyi yönettiğimiz bir konumdayız.

        assert_eq!(found, Err(FindSuspiciesError::NotFound)); //Birşey bulunamaması da bizim için bir Error olması gerektiğinde NotFound ile karşıladık
        assert_eq!(find_suspicies3(query, ";1=1"), Ok(21)); // Birşey bulunursa değerini almak istedik.
        assert_eq!(
            find_suspicies3(query, ""),
            Err(FindSuspiciesError::EmptyWanted) // Aranacak birşey olmaması da bir hataydı.
        );
        assert_eq!(
            find_suspicies3("", "delete from"),
            Err(FindSuspiciesError::EmptyContent) // ve aranan bir şey olmaması da.
        );
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

    Üstteki iki fonksiyonda göz ardı edilen durumlar content veya wanted'ın boş gönderilmesi halleridir.
    Bu hallerde fonksiyondan hata dönülmemektedir.
    Hata durumunu fonksiyon tüketicisine bildirmek için aşağıdaki yol izlenebilir.
*/

// Beklenen hata durumunu kaynağını ile birlikte ifade eden bir Enum tanımlanır
#[derive(Debug, PartialEq)] // assert çağrılarındaki karşılaştırma işlemlerinde kullanabilmek için eklendi.
pub enum FindSuspiciesError {
    EmptyContent,
    EmptyWanted,
    NotFound,
}

// Yeni fonksiyonumuz geriye Result döner.
// Eğer çağıran tarafa bildirmek istediğimiz bir hata varsa bunu FindSuspiciesError enum'ı ile sağlayabiliriz.
pub fn find_suspicies3(content: &str, wanted: &str) -> Result<usize, FindSuspiciesError> {
    // Eğer content değişkeninin uzunluğu sıfır veya küçükse
    if content.len() <= 0 {
        Err(FindSuspiciesError::EmptyContent) // Bir Error nesnesi oluşturulur ve değer olarak FindSuspiciesError::EmptyContent dönülür
    } else if wanted.len() <= 0 {
        Err(FindSuspiciesError::EmptyWanted) // wanted değişkeninin uzunluğu 0 veya küçükse
    } else {
        content
            .find(wanted)
            .map_or(Err(FindSuspiciesError::NotFound), |n| Ok(n)) // diğer durumda da find metodunun sonucuna bakılır. O bir şey bulamadığı durumda FindSuspiciesError::NotFound dönülür. Aksi durumda aranan içeriğin başladığı yer.
    }
}
