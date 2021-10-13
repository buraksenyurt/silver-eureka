fn main() {
    println!("literal match örnekleri");
    println!("15 => {}", match_with_literal(15));
    println!("-80 => {}", match_with_literal(-80));
    println!("76 => {}", match_with_literal(76));
    println!("94 => {}", match_with_literal(94));
    println!("0 => {}", match_with_literal(0));

    println!();
    println!("{}", match_with_tuple((1, 3, 5, 7)));

    println!();
    let dayson = Vehicle {
        pilot_no: 1,
        style: Design::Color(100, 100, 255),
        v_type: VehicleType::Armed,
    };
    println!("{}", match_with_destructuring(dayson));
    let marko = Vehicle {
        pilot_no: 2,
        style: Design::Image("some_logo.png"),
        v_type: VehicleType::Civilian,
    };
    println!("{}", match_with_destructuring(marko));

    // Aşağıdaki Vehicle örneği için match_with_destructuring'teki son seçenek devreye girer
    let jeyna = Vehicle {
        pilot_no: 3,
        style: Design::Image("jeyna.png"),
        v_type: VehicleType::Experimental,
    };
    println!("{}", match_with_destructuring(jeyna));
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

fn match_with_destructuring(v: Vehicle) -> String {
    // parametre olarak gelen Vehicle struct türünün tüm varyasyonlarını ele alacağımız bir match yazabiliriz
    match v {
        // v isimli yapının pilot id, Armed ve Color seçenekleri ile donatılmış olması hali
        Vehicle {
            pilot_no: id,
            v_type: VehicleType::Armed,
            style: Design::Color(r, g, b),
        } => format!(
            "{} nolu pilot için solid (#{:02x}{:02x}{:02x}) renginde araç.",
            id, r, g, b
        ),
        Vehicle {
            // pilot id, Civilian ve Color seçenekleri ile donatılmış olması hali
            pilot_no: id,
            v_type: VehicleType::Civilian,
            style: Design::Color(r, g, b),
        } => format!(
            "{} nolu pilot için solid (#{:02x}{:02x}{:02x}) renginde araç.",
            id, r, g, b
        ),
        Vehicle {
            // pilot id, Image ve Civilian seçenekleri ile donatılmış olması hali(tip önemli değil)
            pilot_no: id,
            v_type: VehicleType::Civilian,
            style: Design::Image(p),
        } => format!("{} nolu pilot için {} logolu araç.", id, p),
        Vehicle {
            // pilot id var ama Experimental ile Image ve Color hallerini ele almadığımız bir nesne gelirse
            pilot_no: id,            
            .. // geri kalanlarının olmaması hali
        } => format!("{} nolu pilot için uygun tasarım oluşturulamadı.", id),
    }
}

enum Design {
    Color(u8, u8, u8),
    Image(&'static str),
}

enum VehicleType {
    Armed,
    Civilian,
    Experimental
}

struct Vehicle {
    style: Design,
    v_type: VehicleType,
    pilot_no: usize,
}
