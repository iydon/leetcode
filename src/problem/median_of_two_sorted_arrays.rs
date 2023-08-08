pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // return Self::find_linear(nums1, nums2);
        return Self::find_log(nums1, nums2);
    }

    #[inline]
    pub fn find_linear(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let size = nums1.len() + nums2.len();
        let count = (size >> 1) + 1;
        let (mut ith, mut jth) = (0, 0);
        let (mut prv, mut cur) = (0, 0);
        while ith + jth < count {
            prv = cur;
            cur = match (nums1.get(ith), nums2.get(jth)) {
                (Some(&num1), Some(&num2)) => {
                    if num1 > num2 {
                        jth += 1;
                        num2
                    } else {
                        ith += 1;
                        num1
                    }
                }
                (Some(&num1), None) => {
                    ith += 1;
                    num1
                }
                (None, Some(&num2)) => {
                    jth += 1;
                    num2
                }
                (None, None) => unreachable!(),
            };
        }
        return if size % 2 == 0 {
            ((prv + cur) as f64) / 2.0
        } else {
            cur as f64
        };
    }

    #[inline]
    pub fn find_log(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let size = nums1.len() + nums2.len();
        let left = Self::_find_log(&nums1, &nums2, 0, 0, (size + 1) / 2);
        let right = Self::_find_log(&nums1, &nums2, 0, 0, (size + 2) / 2);
        return (left + right) as f64 / 2.0;
    }

    fn _find_log(nums1: &Vec<i32>, nums2: &Vec<i32>, ith: usize, jth: usize, kth: usize) -> i32 {
        let (size1, size2) = (nums1.len(), nums2.len());
        let delta = kth / 2;
        if ith >= size1 {
            return nums2[jth + kth - 1];
        }
        if jth >= size2 {
            return nums1[ith + kth - 1];
        }
        if kth == 1 {
            return i32::min(nums1[ith], nums2[jth]);
        }
        let max1 = if (ith + delta - 1) < size1 {
            nums1[ith + delta - 1]
        } else {
            i32::MAX
        };
        let max2 = if (jth + delta - 1) < size2 {
            nums2[jth + delta - 1]
        } else {
            i32::MAX
        };
        return if max1 > max2 {
            Self::_find_log(nums1, nums2, ith, jth + delta, kth - delta)
        } else {
            Self::_find_log(nums1, nums2, ith + delta, jth, kth - delta)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [
            (2.0, vec![1, 3], vec![2]),
            (2.5, vec![1, 2], vec![3, 4]),
            (5.0, vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8]),
            (5.5, vec![1, 3, 5, 7, 9, 11], vec![2, 4, 6, 8]),
        ];
        for (ans, nums1, nums2) in cases {
            assert_eq!(ans, Solution::find_median_sorted_arrays(nums1, nums2));
        }
    }
}
