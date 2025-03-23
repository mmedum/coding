pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut input, 3), 2);

        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut input, 2), 5);

        let mut input = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut input, 3), 2);
    }
}
