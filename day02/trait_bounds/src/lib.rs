// trait bounds sözdiziminin kullanılmasına dair bir örnek

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Önce kendi trait tipimizi oluşturalım
// Herhangi bir tip içindeki bilgileri kullanarak özet bir ifade veren davranış tanımı için kullanacağımızı düşünelim
pub trait Summary {
    fn get_summary(&self) -> String;
}

// Örnek bir veri modeli (Bir blog girdisini temsil ediyor)
pub struct BlogEntry {
    pub title: String,
    pub author: String,
    pub content: String,
}

// Bir başka örnek veri modeli (Bir duyuru metnini temsil etsin)
pub struct Notification {
    pub owner: String,
    pub content: String,
    pub is_argent: bool,
}

// Summary davranışını bu iki tip için uygulayalım
impl Summary for BlogEntry {
    fn get_summary(&self) -> String {
        let short_content: String = self.content.chars().take(100).collect();
        format!("{} {} {}", self.title, self.author, short_content)
    }
}

impl Summary for Notification {
    fn get_summary(&self) -> String {
        format!("{} {}", self.owner, self.content)
    }
}
