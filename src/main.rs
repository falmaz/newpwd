use newpwd::get_random_string;

use clap::Parser;

#[derive(Parser)]
#[command(name = "newpwd")]
#[command(author = "Almaz Fazulzyanov")]
#[command(version = "0.1.0")]
#[command(about = "Generates a random password")]
struct Cli {
    /// Length of the password
    #[arg(short = 'l', long, default_value = "10")]
    length: i32,
}

fn main() {
    let cli = Cli::parse();

    let password = get_random_string(cli.length);
    println!("{}", password.trim());
}