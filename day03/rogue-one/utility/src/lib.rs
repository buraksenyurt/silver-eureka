// workspace içinde kullanılan bir başka sembolik crate
// sadece can_play isimli bir fonksiyon ve testini içeriyor
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_can_play_works_test() {
        assert_eq!(can_play(79), Some(0));
        assert_eq!(can_play(80), Some(1));
        assert_eq!(can_play(0), Some(0));
        assert_eq!(can_play(180), None);
    }
}

pub fn can_play(level_point: u32) -> Option<u8> {
    match level_point {
        0..=79 => Some(0),
        80..=100 => Some(1),
        _ => None,
    }
}
