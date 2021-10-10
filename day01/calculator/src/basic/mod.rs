
pub fn sum(x: f32, y: f32) -> f32 {
    return (x + y) as f32;
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn sum_test() {
        assert_eq!(sum(2.3, 3.5), 5.8);
    }
}
