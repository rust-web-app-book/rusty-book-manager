pub fn sub(a: i32, b: i32) -> i32 {
    a + b // 実装が間違っている
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 0, 10)]
    #[case(100, 5, 95)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(sub(a, b), expected);
    }
}
