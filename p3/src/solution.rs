#![allow(dead_code)]

pub struct Solution();

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = String::new();
        let mut current = String::new();

        for c in s.chars() {
            if let Some(index) = current.clone().find(c) {
                let start_inedx = if index == current.len() {
                    index
                } else {
                    (index) + 1
                };

                current = current[start_inedx..current.len()].to_owned();
                current.push(c);

            } else {
                current.push(c);
            }

            
            if current.len() >= longest.len() {
                longest = current.clone();
            }
        }

        longest.len() as i32
    }
}
