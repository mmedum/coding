use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    for i in 0..s_chars.len() {
        let s_char = s_chars[i];
        let t_char = t_chars[i];

        if s_map.get(&s_char) != t_map.get(&t_char) {
            return false;
        }

        s_map.insert(s_char, i);
        t_map.insert(t_char, i);
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut s = String::from("egg");
        let mut t = String::from("add");
        assert_eq!(is_isomorphic(s, t), true);

        s = String::from("fo");
        t = String::from("bar");
        assert_eq!(is_isomorphic(s, t), false);

        s = String::from("foo");
        t = String::from("bar");
        assert_eq!(is_isomorphic(s, t), false);

        s = String::from("paper");
        t = String::from("title");
        assert_eq!(is_isomorphic(s, t), true);
    }
}
