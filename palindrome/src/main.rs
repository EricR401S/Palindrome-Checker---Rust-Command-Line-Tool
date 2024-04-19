// A command-line palindrome checker
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Eric Rios",
    about = "A simple palindrome checker"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Eric Rios")]
    CheckPalindrome {
        #[clap(short, long)]
        text: String,
    },
}

fn is_palindrome(s: &str) -> bool {
    let normalized = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    normalized == normalized.chars().rev().collect::<String>()
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::CheckPalindrome { text }) => {
            if is_palindrome(&text) {
                println!("'{}' is a palindrome!", text);
            } else {
                println!("'{}' is not a palindrome.", text);
            }
        }
        None => println!("No command given"),
    }
}
