use std::io::{self, Write}; // Imported 'Write' to allow flushing the output stream

fn main() {
    // 1. Changed 'println!' to 'print!' so the user types on the same line
    print!("Enter a number: ");
    
    // Rust buffers output; we must flush it manually to make 'print!' show up immediately
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please type a number!");

    for i in 1..=input {
        println!("{}{}", " ".repeat(input - i), "*".repeat(i));
    }

    // 2. Prevent the .exe from closing immediately
    println!("\nPress Enter to exit...");
    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause); // Waits for the user to press Enter
}
