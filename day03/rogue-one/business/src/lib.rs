// Workspace içine açtığımız sembolik bir modül.
// Player veri yapısı için iki fonksiyon ve test içerir
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_increase_player_works() {
        let mut dragon = Player::new("dragon".to_string(), 56);
        assert_eq!(dragon.to_str(), "dragon(56)");
        dragon.increase_level(4);
        assert_eq!(dragon.level, 60);
    }
}

#[derive(Debug)]
pub struct Player {
    pub nickname: String,
    pub level: u32,
}
impl Player {
    pub fn new(n: String, l: u32) -> Player {
        Player {
            nickname: n,
            level: l,
        }
    }

    pub fn increase_level(&mut self, point: u32) -> &Player {
        self.level += point;
        self
    }

    pub fn to_str(&self) -> String {
        format!("{}({})", self.nickname, self.level)
    }
}
