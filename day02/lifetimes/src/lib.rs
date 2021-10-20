// Her referansın scope bazında bir yaşam ömrü vardır. Bunu belli haller dışında açıkça belirtmemize gerek yoktur.
// Ancak içiçe scope kullanımları referansların fonksiyonlarda taşınması gibi hallerde Rust'ın ödünç alma kontrol mekanizması(borrow cheker) çalışır ve doğru görünen kod derlenmez.
// Bu durumda programcının açıkça(explicit) lifetime kapsamını belirlemesi yani referansa dip not(annotation) eklemesi gerekir.

#[cfg(test)]
mod tests {
    // #[test] // #1 İlk durum
    // fn borrow_cheker_on_play_test() {
    //     let tenant; // reference isimli bir değişken tanımlanır. Yaşam ömrü burada başlıyor. Bunu 'a ile gösterelim.
    //     {
    //         // iç alanda point isimli bir değişken tanımlı. Bunun ömrü sadece bu blok için geçerli. Bunu da 'b ile gösterelim.
    //         let point = 10;
    //         tenant = &point; // point değişkenin bellek referansını tenant isimli değişkene aldık.
    //     } // iç scope'un sonlandığı yer. point değişkeninin ömrü burada bitti. 'b için son. Lakin onun referansını tenant değişkenine almıştık. tenant'a sahiplik verdik dolayısıyla.
    //     assert_eq!(tenant, &10); // artık bu noktada tenant'ın referans ettiği point var olmadığı için tenant'ın bellek mevkisi kaldırılmış bir yeri işaret etme riski söz konusudur.
    //                             // Rust'ın borrow_checker mekanizması bu kullanıma müsaade etmez ve derleme hatası oluşur.
    // } // Burası 'a için yaşam döngüsünün sonlandığı yer.

    #[test] // #2 Birdeki durumun çözümü
    fn valid_borrow_cheker_test() {
        {
            // durumu göstermek için bir scope açıldı. tenant ve point değişkenleri aynı scope içerisindeler. yaşam döngüleri borrow cheker açısından bu scope dahilinde kullanıma müsait.
            let tenant;
            let point = 10;
            tenant = &point;
            assert_eq!(tenant, &10);
        }
    }

    #[test] // #3 Bu senaryoda generic lifetime senaryosu öncesi problemin ne olduğuna bakılıyor
    // Hata alınması Rust derleyici açısında normal ve gereklidir.
    // who_wins fonksiyonuna iki string referansı gider ama hangisinin döneceğini bilemez. (koşula giren string uzunluklarının dinamik geldiğini düşünecek olursak)
    // Ayrıca borrow cheker mekanizması da bu durumu analiz edemez nitekim gelen referanslar ile dönen referansın yaşam ömürlerin arasında ilişkiyi bilemez.
    // Bu sebeple who_wins fonksiyonu değiştirilmeli ve lifetime için dipnot bırakılmaı bir başka deyişler fonksiyon parametre ve dönüş referansları için borrow cheker'ın bu kontrolü atlamasını sağlamalıyız.
    fn need_for_generic_lifetimes_test() {
        let word_1 =
            "Bir berber bir berbere gel bereber berberistanda bir berber dükkanı açalım demiş";
        let word_2="Şu yoğurdu nane ile birlikte sarımsaklasak da mı saklasak nane ile sarımsaklamasak da mı saklamasak. Nanesiz sarımsaklasak da mı saklamasak?";
        let winner = who_wins(word_1, word_2);
        assert_eq!(winner.len(), 140);
    }

    // who_wins fonksiyon string değişkenler için referanslarını alır ki bu son derece doğaldır nitekim büyük String içeriklerin sahipliğini onları yolladığımız fonksiyonun alması önerilmez.
    fn who_wins(my_word: &str, your_word: &str) -> &str {
        if your_word.len() > my_word.len() {
            // ve hangisinin uzunluğu büyükse o referansı fonksiyonun çağırıldığı yere döndürür. Gayet masumane değil mi?
            your_word
        } else {
            my_word
        }
    }
}
