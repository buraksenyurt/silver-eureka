#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mock_add_trait_works() {
        let somewhere = Location::new(3, 4, 5);
        /*
            mock! makrosu ile Location isimli tiple is_exist isimli bir trait eklenmesini
            ve bu işletildiğinde geriye true değer döndürülmesini sağlıyoruz.
        */
        mock!(Location, is_exist, bool, { true });
        let result = somewhere.is_exist();
        assert_eq!(result, true);
    }

    #[test]
    fn should_mock_generate_struct_works() {
        /*
            mock! makrosuna verdiğimiz parametrelerle name, level ve id gibi alanları olan Player isimli bir struct kodunun yazılması sağlanıyor.
            Sonrasında bu struct'ı örnekleyip kullanabiliyoruz.
            İşte rust kodu yazan rust kodu :)
        */
        mock!(Player, name: String, level: u8, id: i32);
        let bobafet = Player {
            name: "boba fet".to_owned(),
            level: 40,
            id: 1001,
        };
        assert_eq!(bobafet.name, "boba fet".to_owned());
        assert_eq!(bobafet.id, 1001);
        assert_eq!(bobafet.level, 40);
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

            Makronun bu dalı ilk parametre ile gelen tipe yeni bir davranış kazandıracak şekilde trait eklenmesini ve
            bu trait için gelen kod bloğunun işletilmesini karşılıyor.
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

        /*
            Aşağıdaki kola uygun bir desen ile karşılaşıltığında ise bir struct yazılıyor.
            İlk parametre struct'ın kimliğini ifade ederken ikinci parametrede sondaki * sembolü sebebiyle
            n sayıda field adı ve tipinin geldiği senaryolar ele alınıyor.
        */
        ($struct_name:ident,$($field_name:ident:$type:ty),*)=>{
            // Struct'ın inşa edildiği yer
            // derive ile birkaç trait davranışını varsayılan olarak ekliyoruz
            #[derive(PartialEq,Clone,Debug)]
            struct $struct_name {
                $(pub $field_name: $type),*
            }
        }
    }
}
