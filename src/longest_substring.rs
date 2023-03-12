//https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut characters: HashSet<char> = HashSet::new();

    let mut left_iter = s.chars().enumerate();
    let mut right_iter = s.chars().enumerate();

    let mut left_index = -1;
    let mut right_index = -1;

    let mut right_char = 'a';

    let mut longest = 0;

    let mut reducing = false;

    loop {
        if reducing {
            match left_iter.next() {
                Some((i, c)) => {
                    characters.remove(&c);

                    if i as i32 == right_index {
                        reducing = false;
                    }
                    if c == right_char {
                        characters.insert(right_char);
                        reducing = false;
                    }

                    left_index = i as i32;
                },
                None => { break; }
            }
        }
        else {
            match right_iter.next() {
                Some((i, c)) => {
                    if characters.contains(&c) {
                        reducing = true;
                    }
                    else {
                        characters.insert(c);
                    }

                    right_index = i as i32;
                    right_char = c;
                },
                None => { break; }
            }
        }

        if !reducing {
            longest = longest.max(right_index - left_index);
        }
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = String::from("abcabcbb");
        let result = length_of_longest_substring(input);
        assert_eq!(result, 3)
    }

    #[test]
    fn example_two() {
        let input = String::from("bbbbb");
        let result = length_of_longest_substring(input);
        assert_eq!(result, 1)
    }

    #[test]
    fn example_three() {
        let input = String::from("pwwkew");
        let result = length_of_longest_substring(input);
        assert_eq!(result, 3)
    }

    #[test]
    fn example_four() {
        let input = String::from("abcbbaxy");
        let result = length_of_longest_substring(input);
        assert_eq!(result, 4)
    }
}
