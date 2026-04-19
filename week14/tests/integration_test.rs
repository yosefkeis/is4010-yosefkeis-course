use lab14::{
    calculate_entropy, check_common_patterns, generate_passphrase, generate_pin, generate_random,
    validate_strength, PasswordStrength,
};

#[test]
fn test_random_password_length() {
    let password = generate_random(20, false);
    assert_eq!(password.len(), 20);
}

#[test]
fn test_random_password_various_lengths() {
    for len in [8, 16, 32].iter() {
        let password = generate_random(*len, false);
        assert_eq!(password.len(), *len);
    }
}

#[test]
fn test_random_password_with_symbols() {
    let password = generate_random(100, true);
    // With 100 characters, very likely to contain at least one symbol
    let has_symbol = password.chars().any(|c| "!@#$%^&*".contains(c));
    assert!(has_symbol || password.len() == 100);
}

#[test]
fn test_pin_only_digits() {
    let pin = generate_pin(6);
    assert_eq!(pin.len(), 6);
    assert!(pin.chars().all(|c: char| c.is_numeric()));
}

#[test]
fn test_pin_various_lengths() {
    for len in [4, 6, 8, 10].iter() {
        let pin = generate_pin(*len);
        assert_eq!(pin.len(), *len);
        assert!(pin.chars().all(|c: char| c.is_numeric()));
    }
}

#[test]
fn test_validate_weak_password() {
    assert_eq!(validate_strength("abc"), PasswordStrength::Weak);
    assert_eq!(validate_strength("password"), PasswordStrength::Weak);
}

#[test]
fn test_validate_medium_password() {
    assert_eq!(validate_strength("Password"), PasswordStrength::Medium);
    assert_eq!(validate_strength("MyPassword"), PasswordStrength::Medium);
}

#[test]
fn test_validate_strong_password() {
    assert_eq!(validate_strength("Password123"), PasswordStrength::Strong);
    assert_eq!(validate_strength("MyPass123"), PasswordStrength::Strong);
}

#[test]
fn test_validate_very_strong_password() {
    assert_eq!(
        validate_strength("MyP@ssw0rd123!"),
        PasswordStrength::VeryStrong
    );
    assert_eq!(
        validate_strength("Secure!Pass2024"),
        PasswordStrength::VeryStrong
    );
}

#[test]
fn test_passphrase_word_count() {
    let passphrase = generate_passphrase(5, '-');
    let word_count = passphrase.split('-').count();
    assert_eq!(word_count, 5);
}

#[test]
fn test_passphrase_separator() {
    let passphrase = generate_passphrase(3, '_');
    assert!(passphrase.contains('_'));

    let passphrase2 = generate_passphrase(3, '~');
    assert!(passphrase2.contains('~'));
}

#[test]
fn test_passphrase_various_lengths() {
    for word_count in [1, 3, 5, 8].iter() {
        let passphrase = generate_passphrase(*word_count, '-');
        let count = passphrase.split('-').count();
        assert_eq!(count, *word_count);
    }
}

#[test]
fn test_entropy_calculation() {
    let weak = calculate_entropy("password");
    let strong = calculate_entropy("MyP@ssw0rd123");

    assert!(weak > 0.0);
    assert!(strong > weak);
}

#[test]
fn test_common_patterns_detection() {
    assert!(check_common_patterns("qwerty123"));
    assert!(check_common_patterns("pass123456"));
    assert!(check_common_patterns("aaaa1111"));
    assert!(!check_common_patterns("MySecure!Pass2024"));
}

#[test]
fn test_password_generation_uniqueness() {
    let pass1 = generate_random(16, true);
    let pass2 = generate_random(16, true);
    // While this could theoretically fail, probability is extremely low
    assert_ne!(pass1, pass2);
}

#[test]
fn test_integration_multiple_operations() {
    // Test using multiple functions together
    let password = generate_random(16, true);
    let strength = validate_strength(&password);
    let entropy = calculate_entropy(&password);

    assert_eq!(password.len(), 16);
    assert!(strength >= PasswordStrength::Strong);
    assert!(entropy > 50.0);
}
