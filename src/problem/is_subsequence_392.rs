pub fn is_subsequence(s: String, t: String) -> bool {
    let (mut i, mut j) = (0, 0);

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    while i < t_chars.len() && j < s_chars.len() {
        if t_chars[i] == s_chars[j] {
            j += 1;
        }

        i += 1;
    }

    return j == s_chars.len();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut subsequence = String::from("abc");
        let mut test = String::from("ahbgdc");
        assert_eq!(is_subsequence(subsequence, test), true);

        subsequence = String::from("axc");
        test = String::from("ahbgdc");
        assert_eq!(is_subsequence(subsequence, test), false);
    }
}
