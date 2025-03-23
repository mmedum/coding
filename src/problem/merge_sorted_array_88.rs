pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);

    while i >= 0 && j >= 0 {
        if nums1[i as usize] >= nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }

        k -= 1;
    }

    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        j -= 1;
        k -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![1, 2, 3, 0, 0, 0];
        merge(&mut input, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(input, vec![1, 2, 2, 3, 5, 6]);

        let mut input = vec![1];
        merge(&mut input, 1, &mut vec![], 0);
        assert_eq!(input, vec![1]);
    }
}
