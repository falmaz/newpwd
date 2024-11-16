//! # Random String Generator
//! This library provides a function to generate a random string of a given length.

use rand::Rng;

/// This function generates a random string of a given length
/// 
/// Example:
/// ```
/// let random_string = get_random_string(10);
/// ```
pub fn get_random_string(length: i32) -> String {
    let chars: Vec<char> = (33..127).map(|i| i as u8 as char).collect();
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    random_string
}
