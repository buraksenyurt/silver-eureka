// Her referansın scope bazında bir yaşam ömrü vardır. Bunu belli haller dışında açıkça belirtmemize gerek yoktur.
// Ancak içiçe scope kullanımları referansların fonksiyonlarda taşınması gibi hallerde Rust'ın ödünç alma kontrol mekanizması(borrow cheker) çalışır ve doğru görünen kod derlenmez.
// Bu durumda programcının açıkça(explicit) lifetime kapsamını belirlemesi yani referansa dip not(annotation) eklemesi gerekir.

#[cfg(test)]
mod tests {
    #[test] // #1 İlk durum
    fn borrow_cheker_on_play_test() {
        let tenant; // reference isimli bir değişken tanımlanır. Yaşam ömrü burada başlıyor. Bunu 'a ile gösterelim.
        {
            // iç alanda point isimli bir değişken tanımlı. Bunun ömrü sadece bu blok için geçerli. Bunu da 'b ile gösterelim.
            let point = 10;
            tenant = &point; // point değişkenin bellek referansını tenant isimli değişkene aldık.
        } // iç scope'un sonlandığı yer. point değişkeninin ömrü burada bitti. 'b için son. Lakin onun referansını tenant değişkenine almıştık. tenant'a sahiplik verdik dolayısıyla.
        assert_eq!(tenant, &5); // artık bu noktada tenant'ın referans ettiği point var olmadığı için tenant'ın bellek mevkisi kaldırılmış bir yeri işaret etme riski söz konusudur.
                                // Rust'ın borrow_checker mekanizması bu kullanıma müsaade etmez ve derleme hatası oluşur.
    } // Burası 'a için yaşam döngüsünün sonlandığı yer.
}
