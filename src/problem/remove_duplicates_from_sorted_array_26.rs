pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;

    for i in 0..nums.len() {
        if k == 0 || nums[i] != nums[k - 1] {
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
        let mut input = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut input), 2);

        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut input), 5);
    }
}
