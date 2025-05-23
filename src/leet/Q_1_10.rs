//! # LeetCode Q1-10 Solutions
//!
//! This module contains solutions for LeetCode problems 1 to 10.

/// 1. Two Sum
///
/// [1. Two Sum](https://leetcode.com/problems/two-sum/)
///
/// Given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.
/// You may assume that each input would have **exactly one solution**, and you may not use the same element twice.
/// You can return the answer in any order.
///
/// ## Example 1
/// ```text
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// ```
///
/// ## Example 2
/// ```text
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// ```
///
/// ## Example 3
/// ```text
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
/// ```
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }

    vec![]
}

/// 2. Add Two Numbers
///
/// [2. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)
///
/// You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// ## Example 1
/// ```text
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
/// ```
///
/// ## Example 2
/// ```text
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
/// ```
///
/// ## Example 3
/// ```text
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Adds two numbers represented by linked lists.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode::new(1)));
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}

/// 3. Longest Substring Without Repeating Characters
///
/// [3. Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
///
/// Given a string `s`, find the length of the **longest** **substring** without duplicate characters.
///
/// ## Example 1
/// ```text
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
/// ```
///
/// ## Example 2
/// ```text
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
/// ```
///
/// ## Example 3
/// ```text
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
/// ```
///
/// ## Constraints
/// - `0 <= s.length <= 5 * 10^4`
/// - `s` consists of English letters, digits, symbols and spaces.

/// 4. Median of Two Sorted Arrays
///
/// [4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)
///
/// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays.
/// The overall run time complexity should be `O(log (m+n))`.
///
/// ## Example 1
/// ```text
/// Input: nums1 = [1,3], nums2 = [2]
/// Output: 2.00000
/// Explanation: merged array = [1,2,3] and median is 2.
/// ```
///
/// ## Example 2
/// ```text
/// Input: nums1 = [1,2], nums2 = [3,4]
/// Output: 2.50000
/// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
/// ```
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut array = nums1;
    let mut arr2 = nums2;
    array.append(&mut arr2);
    array.sort();
    if array.len() % 2 != 0 {
        array[array.len() / 2] as f64
    } else {
        let val = array.len() / 2;
        (array[val - 1] + array[val]) as f64 / 2.0
    }
}

/// 7. Reverse Integer
///
/// [7. Reverse Integer](https://leetcode.com/problems/reverse-integer/)
///
/// Given a signed 32-bit integer `x`, return `x` with its digits reversed. If reversing `x` causes the value to go outside the signed 32-bit integer range `[-2^31, 2^31 - 1]`, then return `0`.
/// **Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**
///
/// ## Example 1
/// ```text
/// Input: x = 123
/// Output: 321
/// ```
///
/// ## Example 2
/// ```text
/// Input: x = -123
/// Output: -321
/// ```
///
/// ## Example 3
/// ```text
/// Input: x = 120
/// Output: 21
/// ```
pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut y: i64 = 0;
    while x != 0 {
        let m = x % 10;
        x /= 10;
        y = (y * 10) + m as i64;
    }
    if y > i32::MAX as i64 || y < i32::MIN as i64 {
        return 0;
    }
    y as i32
}

/// 8. String to Integer
///
/// [8. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)
///
/// Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.
/// The algorithm for `myAtoi(string s)` is as follows:
///
/// 1. **Whitespace**: Ignore any leading whitespace (`" "`).
/// 2. **Signedness**: Determine the sign by checking if the next character is `'-'` or `'+'`, assuming positivity is neither present.
/// 3. **Conversion**: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
/// 4. **Rounding**: If the integer is out of the 32-bit signed integer range `[-2^31, 2^31 - 1]`, then round the integer to remain in the range. Specifically, integers less than `-2^31` should be rounded to `-2^31`, and integers greater than `2^31 - 1` should be rounded to `2^31 - 1`.
///
/// Return the integer as the final result.
///
/// ## Example 1
/// ```text
/// Input: s = "42"
/// Output: 42
/// ```
///
/// ## Example 2
/// ```text
/// Input: s = " -042"
/// Output: -42
/// ```
///
/// ## Example 3
/// ```text
/// Input: s = "1337c0d3"
/// Output: 1337
/// ```
///
/// ## Example 4
/// ```text
/// Input: s = "0-1"
/// Output: 0
/// ```
///
/// ## Example 5
/// ```text
/// Input: s = "words and 987"
/// Output: 0
/// ```
pub fn my_atoi(s: String) -> i32 {
    let mut y = String::new();
    let mut sign = true;
    let mut is_stat = true;
    for char in s.trim().chars() {
        if is_stat && char == '-' {
            is_stat = false;
            sign = false;
        } else if is_stat && char == '+' {
            is_stat = false;
            sign = true;
        } else if char.is_ascii_digit() {
            is_stat = false;
            y.push(char);
        } else {
            break;
        }
    }

    if y.is_empty() {
        return 0;
    }

    match y.parse::<i32>() {
        Ok(num) => {
            if sign {
                num
            } else {
                -num
            }
        }
        Err(_) => {
            if sign {
                i32::MAX
            } else {
                i32::MIN
            }
        }
    }
}

/// 9. Palindrome Number
///
/// [9. Palindrome Number](https://leetcode.com/problems/palindrome-number/)
///
/// Given an integer `x`, return `true` if `x` is a **palindrome**, and `false` otherwise.
///
/// ## Example 1
/// ```text
/// Input: x = 121
/// Output: true
/// Explanation: 121 reads as 121 from left to right and from right to left.
/// ```
///
/// ## Example 2
/// ```text
/// Input: x = -121
/// Output: false
/// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
/// ```
///
/// ## Example 3
/// ```text
/// Input: x = 10
/// Output: false
/// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
/// ```
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    if x % 10 == 0 {
        return false;
    }
    let mut num = 0;
    let mut y = x;
    while y > 0 {
        num = (num * 10) + (y % 10);
        y /= 10;
    }
    x == num
}
