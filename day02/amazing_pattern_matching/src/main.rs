fn main() {
    println!("literal match örnekleri");
    println!("15 => {}", literal_match(15));
    println!("-80 => {}", literal_match(-80));
    println!("76 => {}", literal_match(76));
    println!("94 => {}", literal_match(94));
    println!("0 => {}", literal_match(0));
}

fn literal_match(value: isize) -> String {
    // basit bir literal değerini aşağıdaki gibi dallara ayırıp değerlendirebiliriz
    match value {
        0 => "Başlangıç".to_owned(),              // sıfırsa
        1..=50 => "Amatör seviye".to_owned(),       // 1 - 50 arasında
        51..=70 => "Üst Amatör seviye".to_owned(), // 51 ile 70 arasında
        71..=90 => "Profesyonel adayı".to_owned(),  // 71 ile 90 arasında
        91 => "Profesyonel".to_owned(),              // 91 üstünde
        _ => "Belirsiz".to_owned(), // yukardakilerin hiçbirisine uymayan bir durumda
    }
}
