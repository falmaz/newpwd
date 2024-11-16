use rand::Rng;

pub fn get_random_string(length: i32) -> String {
    let chars: Vec<char> = (33..127).map(|i| i as u8 as char).collect();
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    random_string
}
