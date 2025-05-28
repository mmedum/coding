pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if chars.is_empty() {
        return true;
    }

    let mut l = 0;
    let mut r = chars.len() - 1;

    while l < r {
        if chars[l] != chars[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut input = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(input), true);

        input = String::from("race a car");
        assert_eq!(is_palindrome(input), false);

        input = String::from(" ");
        assert_eq!(is_palindrome(input), true);
    }
}
