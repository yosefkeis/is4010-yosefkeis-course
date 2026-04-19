pub mod generator;
pub mod validator;

// Re-export commonly used items
pub use generator::{generate_passphrase, generate_pin, generate_random};
pub use validator::{
    calculate_entropy, check_common_patterns, validate_strength, PasswordStrength,
};
