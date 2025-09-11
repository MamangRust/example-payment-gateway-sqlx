pub fn mask_card_number(number: &str) -> String {
    let len = number.len();
    if len < 8 {
        "****".to_string()
    } else {
        let prefix = &number[..4];
        let suffix = &number[len - 4..];
        format!("{prefix}****{suffix}")
    }
}

pub fn mask_api_key(key: &str) -> String {
    if key.len() < 10 {
        "****".to_string()
    } else {
        let (prefix, suffix) = key.split_at(6);
        let suffix = &suffix[suffix.len() - 4..];
        format!("{prefix}...{suffix}")
    }
}
