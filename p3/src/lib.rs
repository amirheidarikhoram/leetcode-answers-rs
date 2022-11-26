mod solution;

#[cfg(test)]
mod tests {
    use crate::solution::*;

    #[test]
    fn it_works() {
        let test_case = String::from("abcabcbb");

        assert_eq!(Solution::length_of_longest_substring(test_case), 3);
    }
}
