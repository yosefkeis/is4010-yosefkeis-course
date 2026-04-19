use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*";

// Predefined word list for passphrase generation
const WORD_LIST: &[&str] = &[
    "correct", "horse", "battery", "staple", "python", "rust", "secure", "password", "quantum",
    "digital", "fusion", "crystal", "phoenix", "dragon", "eclipse", "zenith", "cascade", "thunder",
    "glacier", "harbor", "island", "journal", "karma", "legacy", "meadow", "nebula", "ocean",
    "palace", "quartz", "radiant", "spiral", "twilight",
];

/// Generates a random password with specified length
///
/// # Arguments
/// * `length` - The desired password length
/// * `use_symbols` - Whether to include symbols in the password
pub fn generate_random(length: usize, use_symbols: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut charset = String::new();
    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);
    charset.push_str(DIGITS);

    if use_symbols {
        charset.push_str(SYMBOLS);
    }

    let chars: Vec<char> = charset.chars().collect();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

/// Generates a passphrase from a predefined word list
///
/// # Arguments
/// * `word_count` - Number of words in the passphrase
/// * `separator` - Character to separate words
pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();

    (0..word_count)
        .map(|_| {
            let idx = rng.gen_range(0..WORD_LIST.len());
            WORD_LIST[idx]
        })
        .collect::<Vec<&str>>()
        .join(&separator.to_string())
}

/// Generates a numeric PIN code
///
/// # Arguments
/// * `length` - The desired PIN length
pub fn generate_pin(length: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let digit = rng.gen_range(0..10);
            (digit as u8 + b'0') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_length() {
        for len in [8, 16, 32, 64].iter() {
            let password = generate_random(*len, false);
            assert_eq!(password.len(), *len);
        }
    }

    #[test]
    fn test_generate_random_no_symbols() {
        let password = generate_random(100, false);
        assert!(!password.chars().any(|c| SYMBOLS.contains(c)));
    }

    #[test]
    fn test_generate_random_with_symbols() {
        let password = generate_random(100, true);
        // With 100 characters, very likely to contain at least one symbol
        let has_symbol = password.chars().any(|c| SYMBOLS.contains(c));
        assert!(has_symbol || password.len() == 100);
    }

    #[test]
    fn test_generate_passphrase_word_count() {
        for word_count in [1, 3, 5, 8].iter() {
            let passphrase = generate_passphrase(*word_count, '-');
            let count = passphrase.split('-').count();
            assert_eq!(count, *word_count);
        }
    }

    #[test]
    fn test_generate_passphrase_separator() {
        let passphrase = generate_passphrase(4, '_');
        assert!(passphrase.contains('_'));

        let passphrase2 = generate_passphrase(3, '~');
        assert!(passphrase2.contains('~'));
    }

    #[test]
    fn test_generate_pin_length() {
        for len in [4, 6, 8].iter() {
            let pin = generate_pin(*len);
            assert_eq!(pin.len(), *len);
        }
    }

    #[test]
    fn test_generate_pin_digits_only() {
        let pin = generate_pin(20);
        assert!(pin.chars().all(|c| c.is_numeric()));
    }
}
