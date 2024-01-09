use rand::{thread_rng, Rng};




pub fn generate_random_string(characters: &str) -> String {
    let mut rng = rand::thread_rng();
    (0..16)
        .map(|_| characters.chars().nth(rng.gen_range(0..characters.len())).unwrap())
        .collect()
}
