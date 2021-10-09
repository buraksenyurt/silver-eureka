#[derive(Clone, Debug, Copy)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    pub fn new(_x: i32, _y: i32, _z: i16) -> Location {
        Location {
            x: _x,
            y: _y,
            z: _z as i32,
        }
    }
    pub fn sum(&self) -> i32 {
        self.x + self.y + self.z as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_math() {
        assert_eq!(2 * 6, 12);
        assert_eq!(2.22 + 2.22, 4.44);
        assert_eq!(2_i32.pow(3), 8);
        assert_eq!(16_f32.sqrt(), 4_f32);

        let x: u64 = 128;
        let y: u64 = 256;
        assert_eq!(y - x, 128);

        let mut counter = 1;
        counter += 1;
        assert_eq!(counter, 2);
    }

    // #[test]
    // #[should_panic]
    // fn overflows() {
    //     let x: u64 = 128;
    //     let y = 256_u64;
    //     let _ = x - y;
    // }

    use super::Location;

    #[test]
    fn location_test() {
        let loc = Location::new(4, 1, 6);
        assert_eq!(loc.x, 4);
        assert_eq!(loc.y, 1);
        assert_eq!(loc.z, 6);
        assert_eq!(loc.sum(), 11);

        let shadow = loc.clone();
        assert_eq!(shadow.z, 6);

        let mut sprit = loc;
        sprit.x = 8;
        assert_eq!(loc.x, 4);
        assert_eq!(sprit.x, 8);
    }
}
