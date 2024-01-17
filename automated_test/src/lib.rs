fn _get_string_len(s: &str) -> i32
{
    let mut num = 0;
    for _i in s.as_bytes() {
        num+=1;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_length() {
        let result = _get_string_len("Hello");
        assert_eq!(result, 5);
    }
}
