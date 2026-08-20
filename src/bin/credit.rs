use std::io::{self, Write};

// fn no_digit_check(n: usize) -> bool {
//     let digits = n.checked_ilog10().unwrap_or(0) + 1;
//     digits>=13 
// }

fn  type_of_card(n: usize, l_check: bool) {
    if l_check {
        let digits = n.checked_ilog10().unwrap_or(0) + 1;
        if digits == 15 && (n / 10_usize.pow(13) == 34 || n / 10_usize.pow(13) == 37) {
            println!("AMEX");
        } else if digits == 16 && (n / 10_usize.pow(14) >= 51 && n / 10_usize.pow(14) <= 55) {
            println!("MASTERCARD");
        } else if (digits == 13 || digits == 16) && n / 10_usize.pow(digits as u32 - 1) == 4 {
            println!("VISA");
        } else {
            println!("Unknown card type/Invalid card type.");
        }
    }
    else {
        println!("Invalid card type.");
    }
}
// AMEX: 15 digits, starts with 34 or 37
// MASTERCARD: 16 digits, starts with 51–55
// VISA: 13 or 16 digits, starts with 4

fn luhns_check(mut n: usize/* , d_check: bool*/) -> bool {
    let mut position = 1;
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        if position % 2 == 0 {
            // println!("digit {} is at an EVEN position — double it", digit);
        //     if matches!(n.abs(), 10..=99) {
        //         let mut d1 = digit*2 % 10;
        //         let mut d2 = digit*2 / 10;
        //         sum = sum + d1 + d2;
        // }
        // sum = sum + digit * 2;
            let doubled = digit * 2;
            if doubled > 9 {
                let d1 = doubled % 10;
                let d2 = doubled / 10;
                sum = sum + d1 + d2;
            } else {
                sum = sum + doubled;
            }
    }
        else {
            // println!("digit {} is at an ODD position — keep it", digit);
            sum = sum + digit;
        }
        n /= 10;
        position += 1;
    }
    // println!("Sum of processed digits: {}", sum);
    if sum % 10 == 0 /*&& d_check*/ {
        return true;
    } else {
        return false;
    }
}

fn main(){
    print!("Enter a number to check with Luhn's algorithm: ");
     let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: usize = input.trim().parse().expect("Please type a number!");
    // let digit_check = no_digit_check(input);
    let luhn_check = luhns_check(input/* , digit_check*/);
    type_of_card(input, luhn_check);
    println!("\nPress Enter to exit...");
    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause);
}