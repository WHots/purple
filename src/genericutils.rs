use std::char;
use rand::Rng;

pub fn generate_random_string(characters: &str) -> String {

    let mut rng = rand::thread_rng();
    
    (0..16)
        .map(|_| characters.chars().nth(rng.gen_range(0..characters.len())).unwrap_or_else(|| {
            char::from_digit(rng.gen_range(0..26) as u32 % 26 + 10, 36).unwrap()
        }))
        .collect()
}