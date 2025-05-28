pub fn word_pattern(pattern: String, s: String) -> bool {


    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut pattern = String::from("abba");
        let mut s = String::from("dog cat cat dog");
        assert_eq!(word_pattern(pattern, s), true);

        pattern = String::from("abba");
        s = String::from("dog cat cat fish");
        assert_eq!(word_pattern(pattern, s), false);

        pattern = String::from("aaaa");
        s = String::from("dog cat cat dog");
        assert_eq!(word_pattern(pattern, s), false);
    }
}
