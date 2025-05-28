use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut elements = HashMap::<i32, i32>::new();

    for num in nums {
        elements
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let (mut current_majority, mut current_largest_count) = (0, -1);
    for (key, value) in elements.iter() {
        if *value >= current_largest_count {
            current_majority = *key;
            current_largest_count = *value;
        }
    }

    current_majority
}

pub fn majority_element_boyer_moore(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut majority = 0;

    for num in nums {
        if count == 0 {
            majority = num;
            count = 1;
        } else if majority == num {
            count += 1;
        } else if majority != num {
            count -= 1;
        }
    }

    majority
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![3, 2, 3];
        assert_eq!(majority_element(input), 3);

        let input = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(input), 2);
    }

    #[test]
    fn test_boyer_moore() {
        let input = vec![3, 2, 3];
        assert_eq!(majority_element_boyer_moore(input), 3);

        let input = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element_boyer_moore(input), 2);
    }
}
