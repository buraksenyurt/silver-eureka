use rand::prelude::*;

pub fn get_location() -> Location {
    let x: f32 = random::<f32>();
    let y: f32 = random::<f32>();
    let loc = Location::new(x, y);
    return loc;
}

#[derive(Debug)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(_x: f32, _y: f32) -> Location {
        Location { x: _x, y: _y }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_location_test() {
        let loc = get_location();
        assert!(loc.x > 0.0);
        assert!(loc.y > 0.0);
    }
}
