use clap::{Parser, Subcommand};
use lab14::{
    calculate_entropy, check_common_patterns, generate_passphrase, generate_pin, generate_random,
    validate_strength,
};

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "A secure password generator and validator", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password
    Random {
        /// Password length
        #[arg(short, long, default_value_t = 16)]
        length: usize,

        /// Include symbols in password
        #[arg(short, long)]
        symbols: bool,
    },

    /// Generate a passphrase from words
    Passphrase {
        /// Number of words
        #[arg(short, long, default_value_t = 4)]
        words: usize,

        /// Word separator character
        #[arg(short, long, default_value = "-")]
        separator: char,
    },

    /// Generate a numeric PIN
    Pin {
        /// PIN length
        #[arg(short, long, default_value_t = 4)]
        length: usize,
    },

    /// Validate password strength
    Validate {
        /// Password to validate
        password: String,

        /// Show detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            let password = generate_random(length, symbols);
            println!("Generated password: {}", password);
            println!("Length: {}", password.len());

            let entropy = calculate_entropy(&password);
            println!("Entropy: {:.2} bits", entropy);

            let strength = validate_strength(&password);
            println!("Strength: {:?}", strength);
        }

        Commands::Passphrase { words, separator } => {
            let passphrase = generate_passphrase(words, separator);
            println!("Generated passphrase: {}", passphrase);
            println!("Word count: {}", passphrase.split(separator).count());

            let entropy = calculate_entropy(&passphrase);
            println!("Entropy: {:.2} bits", entropy);
        }

        Commands::Pin { length } => {
            let pin = generate_pin(length);
            println!("Generated PIN: {}", pin);
            println!("Length: {}", pin.len());
        }

        Commands::Validate { password, detailed } => {
            let strength = validate_strength(&password);
            println!("Password strength: {:?}", strength);

            if detailed {
                let entropy = calculate_entropy(&password);
                println!("Length: {}", password.len());
                println!("Entropy: {:.2} bits", entropy);

                let has_lower = password.chars().any(|c| c.is_lowercase());
                let has_upper = password.chars().any(|c| c.is_uppercase());
                let has_digit = password.chars().any(|c| c.is_numeric());
                let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

                println!("\nCharacter types:");
                println!("  Lowercase: {}", if has_lower { "Yes" } else { "No" });
                println!("  Uppercase: {}", if has_upper { "Yes" } else { "No" });
                println!("  Digits: {}", if has_digit { "Yes" } else { "No" });
                println!("  Symbols: {}", if has_symbol { "Yes" } else { "No" });

                let has_patterns = check_common_patterns(&password);
                println!(
                    "  Common patterns: {}",
                    if has_patterns { "Yes" } else { "No" }
                );
            }
        }
    }
}
