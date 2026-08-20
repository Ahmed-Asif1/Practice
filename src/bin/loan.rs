use std::io::{self, Write};
fn main() {
    print!("Enter your age: ");
    let _ = io::stdout().flush();
    let age = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<u32>().expect("Please type a valid age!")
    };
    let monthly_income = {
        print!("Enter your monthly income: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<u32>().expect("Please type a valid income!")
    };
    let credit_score = {
        print!("Enter your credit score: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<u32>().expect("Please type a valid credit score!")
    };
    let employed = {
        print!("Are you employed? (yes/no): ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "yes" => true,
            "no" => false,
            _ => {
                println!("Invalid input for employment status. Assuming 'no'.");
                false
            }
        }
    };

    if age < 21 || age > 60 {
        println!("Rejected: Age is not between 21 and 60.");
    } else if monthly_income < 80_000 {
        println!("Rejected: Income is less than 80,000.");
    } else if credit_score < 650 {
        println!("Rejected: Credit score is less than 650.");
    } else if !employed {
        println!("Rejected: Applicant is not employed.");
    } else {
        println!("Loan Approved.");
    }
}