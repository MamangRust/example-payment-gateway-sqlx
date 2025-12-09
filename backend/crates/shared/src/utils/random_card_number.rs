use rand::{Rng, rng};
use regex::Regex;

pub fn random_card_number() -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rng();

    let random_digits: String = (0..15)
        .map(|_| rng.random_range(0..10).to_string())
        .collect();

    let candidate = format!("4{random_digits}");

    let re = Regex::new(r"^\d{16}$")?;
    if re.is_match(&candidate) {
        Ok(candidate)
    } else {
        Err("Generated number is invalid".into())
    }
}
