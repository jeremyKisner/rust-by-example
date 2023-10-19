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
    let strings_in = &strs;
    let common = "";
    for s in strings_in {
        println!("I like {}", s)
    }
    return common.to_string()
}

fn main() {
    println!("Hello, world!");
    let input: Vec<String> = vec![
    String::from("flower"),
    String::from("flow"),
    String::from("flight"),
];
    let result = longest_common_prefix(input);
    println!("Longest common prefix: {}", result)
}
