use std::io;

fn format_and_join(input: &str) -> String {
    let formatted = format!("^{}$", input);

    // Use fold to intersperse '#' between characters
    formatted.chars().fold(String::new(), |mut acc, c| {
        if !acc.is_empty() {
            acc.push('#');
        }
        acc.push(c);
        acc
    })
}

fn main() {
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // Apply format_and_join to the input string
    let transformed = format_and_join(&user_input);

    // Initialize variables
    let length = transformed.len();
    let mut palindrome_lengths: Vec<i32> = vec![0; length]; // Vector to store palindrome lengths
    let (mut center, mut right_boundary) = (0, 0); // Center and right boundary of the current palindrome

    // Iterate over the characters of the transformed string
    for current_index in 1..length - 1 {
        if right_boundary > current_index {
            palindrome_lengths[current_index] = (right_boundary - current_index).min(palindrome_lengths[2 * center - current_index] as usize) as i32;
        }

        // Expand the palindrome centered at current_index
        while ((current_index + 1 + palindrome_lengths[current_index] as usize) < length)
            && (current_index >= 1 + palindrome_lengths[current_index] as usize)
            && (transformed.chars().nth(current_index + 1 + palindrome_lengths[current_index] as usize) == transformed.chars().nth(current_index - 1 - palindrome_lengths[current_index] as usize))
        {
            palindrome_lengths[current_index] += 1;
        }

        // Adjust the center and right boundary if necessary
        if current_index + palindrome_lengths[current_index] as usize > right_boundary {
            center = current_index;
            right_boundary = current_index + palindrome_lengths[current_index] as usize;
        }
    }

    // Find the longest palindrome in the original string
    let (mut max_length, mut center_index) = (0, 0);
    for current_index in 1..length - 1 {
        if palindrome_lengths[current_index] > max_length {
            max_length = palindrome_lengths[current_index];
            center_index = current_index;
        }
    }

    // Extract the longest palindrome substring from the original input
    let start_index = (center_index - max_length as usize) / 2;
    let end_index = start_index + max_length as usize;
    let longest_palindrome = &user_input[start_index..end_index];

    println!("{}", longest_palindrome);
}