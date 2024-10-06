use std::io;
// Function to transform string to format. It needs to be done because it will reduce the calculation of an array for even lengths to an odd one
fn transform(string: &str) -> Vec<u8> {
    let len_string = string.len();

    let mut transformed_vector = Vec::with_capacity(2 * len_string + 3); // 2 * n + 3 because we adding 2n+1 '#' and one '^' and '$' each
    transformed_vector.push(b'^');
    transformed_vector.push(b'#');
    for byte in string.as_bytes() {
        transformed_vector.push(*byte);
        transformed_vector.push(b'#');
    }
    transformed_vector.push(b'$');
    transformed_vector
}

fn manacher(transformed: &Vec<u8>) -> Vec<usize> {
    let length = transformed.len();
    let mut palindrome_lengths = vec![0; length];
    let (mut center, mut right_boundary) = (0, 0);
    //Manacher's algorithm
    for current_index in 1..length - 1 {
        if current_index < right_boundary {
            palindrome_lengths[current_index] = usize::min(
                right_boundary - current_index,
                palindrome_lengths[2 * center - current_index],
            );
        }
        while transformed[current_index + 1 + palindrome_lengths[current_index]]
            == transformed[current_index - 1 - palindrome_lengths[current_index]]
        {
            palindrome_lengths[current_index] += 1;
        }

        if current_index + palindrome_lengths[current_index] > right_boundary {
            center = current_index;
            right_boundary = current_index + palindrome_lengths[current_index];
        }
    }
    palindrome_lengths
}

fn longest_palindrome(palindrome_lengths: &Vec<usize>) -> (usize, usize) {
    let (max_len, center_index) =
        palindrome_lengths
            .iter()
            .enumerate()
            .fold(
                (0, 0),
                |acc, (i, &len)| {
                    if len > acc.0 {
                        (len, i)
                    } else {
                        acc
                    }
                },
            );
    (max_len, center_index)
}

fn main() {
    // Read input
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    // Transform input to format
    let transformed = transform(&user_input);
    //Manacher's algorithm
    let palindrome_lengths = manacher(&transformed);
    // Find longest palindrome
    let (max_len, center_index) = longest_palindrome(&palindrome_lengths);
    // Print longest palindrome
    let start = (center_index - max_len) / 2;
    println!("{}", &user_input[start..start + max_len as usize]);
}
