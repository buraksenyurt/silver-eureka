#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mock_works() {
        let somewhere = Location::new(3, 4, 5);
        /*
            mock! makrosu ile Location isimli tiple is_exist isimli bir trait eklenmesini
            ve bu işletildiğinde geriye true değer döndürülmesini sağlıyoruz.
        */
        mock!(Location, is_exist, bool, { true });
        let result = somewhere.is_exist();
        assert_eq!(result, true);
    }
}

pub struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    pub fn new(x: i32, y: i32, z: i32) -> Location {
        Location { x: x, y: y, z: z }
    }
}

#[macro_use]
mod macromania {
    #[macro_export]
    macro_rules! mock {
        /*
            İlk parametre ile bir tip alıyoruz.
            İkinci parametre identifier için.
            Üçüncü parametre dönüş tipleri için kullanılmakta.
            Son parametre ise kod bloğu almakta.

            Bu makro gelen argümanlarına göre gelen tipe yeni bir davranış kazandıracak şekilde bir trait eklenmesini ve
            bu trait için gelen kod bloğunun uygulanmasını sağlıyor.
        */
        ($type:ty,$trait_name:ident,$return_type:ty,$code_block:block) => {

            // trait tanımı
            pub trait $trait_name{
                fn $trait_name(&self)->$return_type;
            }

            // trait'in uygulandığı kod bloğu tanımı
            impl $trait_name for $type{
                fn $trait_name(&self) -> $return_type $code_block
            }
        };
    }
}
