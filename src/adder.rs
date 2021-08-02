pub(crate) fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        let sum = add(2, 2);
        let expected = 4;

        assert_eq!(expected, sum);
    }
}
