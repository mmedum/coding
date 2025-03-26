use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut m_count: HashMap<char, i32> = HashMap::new();

    for c in magazine.chars() {
        m_count
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for c in ransom_note.chars() {
        match m_count.get_mut(&c) {
            Some(count) if *count > 0 => {
                *count -= 1;
            }
            _ => {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut ransom_note = String::from("a");
        let mut magazine = String::from("b");
        assert_eq!(can_construct(ransom_note, magazine), false);

        ransom_note = String::from("aa");
        magazine = String::from("ab");
        assert_eq!(can_construct(ransom_note, magazine), false);

        ransom_note = String::from("aa");
        magazine = String::from("aab");
        assert_eq!(can_construct(ransom_note, magazine), true);

        ransom_note = String::from("aab");
        magazine = String::from("baa");
        assert_eq!(can_construct(ransom_note, magazine), true);
    }
}
