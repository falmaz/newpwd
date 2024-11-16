use newpwd::get_random_string;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_string() {
        let length = 10;
        let result = get_random_string(length);
        assert_eq!(result.len(), length as usize);
        // Verify characters are printable ASCII
        for c in result.chars() {
            assert!(c as u8 >= 33 && c as u8 <= 126);
        }
    }
}