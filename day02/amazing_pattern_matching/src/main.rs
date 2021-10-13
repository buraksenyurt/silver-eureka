fn main() {
    println!("literal match örnekleri");
    println!("15 => {}", match_with_literal(15));
    println!("-80 => {}", match_with_literal(-80));
    println!("76 => {}", match_with_literal(76));
    println!("94 => {}", match_with_literal(94));
    println!("0 => {}", match_with_literal(0));

    println!();
    println!("{}", match_with_tuple((1, 3, 5, 7)));
}

fn match_with_tuple(points: (i32, i32, i32, i32)) -> String {
    // tuple veri türü üstünde de pattern matching etkili şekilde kullanılabilir
    // örneğin points isimli tuple'daki ilk ve son sayıları atlayıp diğerlerini kullanabiliriz
    match points {
        (_, second, third, _) => format!("{}...{}", second, third),
    }
}

fn match_with_literal(value: isize) -> String {
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
