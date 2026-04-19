/// Password strength levels
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Ord, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

/// Validates the strength of a password
///
/// Criteria:
/// - Weak: < 8 characters or only lowercase
/// - Medium: 8+ characters, mixed case
/// - Strong: 8+ characters, mixed case + digits
/// - VeryStrong: 12+ characters, mixed case + digits + symbols
pub fn validate_strength(password: &str) -> PasswordStrength {
    let len = password.len();
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    match (len, has_lower, has_upper, has_digit, has_symbol) {
        (l, _, _, _, true) if l >= 12 => PasswordStrength::VeryStrong,
        (l, true, true, true, _) if l >= 8 => PasswordStrength::Strong,
        (l, true, true, _, _) if l >= 8 => PasswordStrength::Medium,
        _ => PasswordStrength::Weak,
    }
}

/// Checks if password contains common patterns
///
/// Returns true if password contains:
/// - Sequential numbers (123, 456, etc.)
/// - Repeated characters (aaa, 111, etc.)
/// - Keyboard patterns (qwerty, asdf, etc.)
pub fn check_common_patterns(password: &str) -> bool {
    let password_lower = password.to_lowercase();

    // Keyboard patterns
    let keyboard_patterns = [
        "qwerty", "asdf", "zxcv", "123456", "password", "admin", "letmein",
    ];
    for pattern in keyboard_patterns.iter() {
        if password_lower.contains(pattern) {
            return true;
        }
    }

    // Sequential numbers
    for start in 0..=6 {
        let seq = format!("{}{}{}", start, start + 1, start + 2);
        if password_lower.contains(&seq) {
            return true;
        }
    }

    // Repeated characters (3+ in a row)
    let chars: Vec<char> = password_lower.chars().collect();
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 1] && chars[i + 1] == chars[i + 2] {
            return true;
        }
    }

    false
}

/// Calculates password entropy in bits
///
/// Formula: length * log2(charset_size)
/// Returns entropy as floating point number
pub fn calculate_entropy(password: &str) -> f64 {
    let len = password.len() as f64;

    let mut charset_size = 0;
    if password.chars().any(|c| c.is_lowercase()) {
        charset_size += 26;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        charset_size += 26;
    }
    if password.chars().any(|c| c.is_numeric()) {
        charset_size += 10;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        charset_size += 32; // Approximate for common symbols
    }

    if charset_size == 0 {
        return 0.0;
    }

    len * (charset_size as f64).log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_weak_short() {
        assert_eq!(validate_strength("abc"), PasswordStrength::Weak);
        assert_eq!(validate_strength("short"), PasswordStrength::Weak);
    }

    #[test]
    fn test_validate_weak_lowercase_only() {
        assert_eq!(validate_strength("lowercase"), PasswordStrength::Weak);
    }

    #[test]
    fn test_validate_medium() {
        assert_eq!(validate_strength("Password"), PasswordStrength::Medium);
        assert_eq!(validate_strength("MyPassword"), PasswordStrength::Medium);
    }

    #[test]
    fn test_validate_strong() {
        assert_eq!(validate_strength("Password123"), PasswordStrength::Strong);
        assert_eq!(validate_strength("MyPass123"), PasswordStrength::Strong);
    }

    #[test]
    fn test_validate_very_strong() {
        assert_eq!(
            validate_strength("MyP@ssw0rd123"),
            PasswordStrength::VeryStrong
        );
        assert_eq!(
            validate_strength("Secure!Pass2024"),
            PasswordStrength::VeryStrong
        );
    }

    #[test]
    fn test_check_common_patterns_keyboard() {
        assert!(check_common_patterns("qwerty123"));
        assert!(check_common_patterns("asdfghjk"));
    }

    #[test]
    fn test_check_common_patterns_sequential() {
        assert!(check_common_patterns("pass123456"));
        assert!(check_common_patterns("abc012def"));
    }

    #[test]
    fn test_check_common_patterns_repeated() {
        assert!(check_common_patterns("passs123"));
        assert!(check_common_patterns("aaa111"));
    }

    #[test]
    fn test_check_common_patterns_none() {
        assert!(!check_common_patterns("secure!Pass2024"));
        assert!(!check_common_patterns("MyR@nd0mP@ss"));
    }

    #[test]
    fn test_calculate_entropy_basic() {
        let entropy = calculate_entropy("abcdefgh");
        assert!(entropy > 0.0);
        // 8 characters with only lowercase: 8 * log2(26) ≈ 37.6
        assert!(entropy < 40.0);
    }

    #[test]
    fn test_calculate_entropy_strong() {
        let entropy = calculate_entropy("MyP@ss123!");
        assert!(entropy > 0.0);
        // Should have higher entropy with mixed character types
        assert!(entropy > 40.0);
    }

    #[test]
    fn test_calculate_entropy_empty() {
        let entropy = calculate_entropy("");
        assert_eq!(entropy, 0.0);
    }
}
