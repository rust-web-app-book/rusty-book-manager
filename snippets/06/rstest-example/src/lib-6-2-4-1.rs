pub fn sub(a: i32, b: i32) -> i32 {
    a + b // 実装が間違っている
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub_1() {
        assert_eq!(sub(10, 0), 10);
    }
    #[test]
    fn test_sub_2() {
        assert_eq!(sub(100, 5), 95);
    }
}
