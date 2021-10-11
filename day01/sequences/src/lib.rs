#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn test_for_arrays() {
        let mut numbers: [usize; 4] = [1; 4]; // 4 tane 1 sayısından oluşan bir dizi tanımı. mutable.
        assert_eq!(numbers, [1, 1, 1, 1]);
        numbers[0] = 0;
        assert_eq!(numbers, [0, 1, 1, 1]);
        let numbers_copy: [usize; 4] = [3, 5, 7, 9];
        assert_eq!(numbers_copy, [3, 5, 7, 9]);
        assert_eq!(mem::size_of_val(&numbers), mem::size_of::<usize>() * 4); // unsigned int üstünden boyut kontrolü. dizinin boyutu usize ile eleman sayısının çarpımını eşittir
    }

    #[test]
    fn test_for_tuples() {
        let mut basket: (f32, usize, String, i16) = (3.14, 32, "Rust Cookbook".to_string(), -48); // farklı türden elemanlar barındıran Tuple tanımı
        assert_eq!(basket.3, -48); // Tuple elamanlarına sayısal indis değerleri ile erişilir
        assert_eq!(basket.2, "Rust Cookbook");
        basket.0 = 3.1415;
        assert_eq!(basket.0, 3.1415);

        let (_item1, _item2, _item3, _) = basket; // Bir tuple başka bir tuple'a bu şekilde atanabilir(unpack işlemi). Sondaki elemanı _ ile işleme dahil etmedik
        assert_eq!(_item1, 3.1415);

        let complex1 = Complex(3, 4); // Struct veri türü de tuple üstüne kuruludur. O nedenle alttaki Complex tanımına göre 0,1 ile elemanlarına erişilebilir.
        assert_eq!(complex1.0, 3);
        assert_eq!(complex1.1, 4);
        let (_real, _virtual) = (complex1.0, complex1.1);
        assert_eq!(_real, 3);
        assert_eq!(_virtual, 4);
    }

    #[test]
    fn test_for_vectors() {
        let mut vector1: Vec<i32> = vec![1, 2, 3, 4]; // İlk elemanları belirleyerek yapılan bir vector tanımı
        assert_eq!(vector1, [1, 2, 3, 4]);
        vector1.push(5); // vector'e yeni bir eleman ekleme
        assert_eq!(vector1, [1, 2, 3, 4, 5]);
        assert_eq!(mem::size_of::<Vec<i32>>(), mem::size_of::<usize>() * 3); // Eleman içermeyen bir vektor tanımlandığında varsayılan olarak üç bilgi içerir. pointer, kapasite ve uzunluk. Dolayısıya bellekte kapladığı alan usize*3 olur
        assert_eq!(vector1[1], 2); // Vector elemanlarına indis kullanarak erişilebilir
        vector1.insert(0, 6); // insert ile belli bir konuma eleman eklenebilir
        vector1.insert(1, 7);
        assert_eq!(vector1, [6, 7, 1, 2, 3, 4, 5]);
        let last = vector1.pop(); // pop ile son sıradaki eleman vektörden çekilerek çıkarılır.
        assert_eq!(last, Some(5));
        assert_eq!(vector1, [6, 7, 1, 2, 3, 4]);
        vector1.remove(1); // 1nci indisteki eleman çıkartılır
        assert_eq!(vector1, [6, 1, 2, 3, 4]);

        let mut vector2 = Vec::with_capacity(5); // Başlangıçta eleman kapasitesi belirlenebilir
        vector2.extend([6, 8, 2]);  // Sonrasında elemanlar eklenebilir
        assert_eq!(vector2, [6, 8, 2]);
        assert_eq!(vector2.capacity(), 5); 
        vector2.shrink_to_fit(); // Kapasite kaç eleman kullanıyorsak ona göre düzeltilebilir
        assert_eq!(vector2.capacity(), 3);
    }

    struct Complex(i32, i32);
}
