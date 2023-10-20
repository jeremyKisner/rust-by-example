// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new(); // If the input vector is empty, return an empty string.
    }

    // Find the minimum length string in the vector
    let min_len = strs.iter().map(|s| s.len()).min().unwrap();

    let mut prefix = String::new();
    for i in 0..min_len {
        let char_at_i = strs[0].chars().nth(i).unwrap(); // Get the character at index i from the first string
        if strs.iter().all(|s| s.chars().nth(i) == Some(char_at_i)) {
            prefix.push(char_at_i);
        } else {
            break; // If the characters don't match for all strings, break the loop.
        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix_with_common() {
        let input: Vec<String> = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(input), "fl");
    }

    #[test]
    fn test_longest_common_prefix_uncommon() {
        let input: Vec<String> = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(longest_common_prefix(input), "");
    }
}

fn main() {
    let input: Vec<String> = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let result = longest_common_prefix(input);
    println!("Longest common prefix: {}", result)
}
