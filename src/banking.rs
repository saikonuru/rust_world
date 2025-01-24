use std::io;
use std::io::Write;
// use std::{io, ops::ControlFlow};

fn main() {
    let mut balance: f64 = 0.0;
    loop {
        println!("Please choose an option");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Balance");
        println!("Any other key exit");
        let input = get_user_input("Please your choice: ");

        match input.trim().parse::<u32>() {
            Ok(1) => fn_deposit(&mut balance),
            Ok(2) => fn_withdraw(&mut balance),
            Ok(3) => println!("Balance: {}", balance),
            _ => {
                println!("Thank you!");
                break;
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input_str = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read input");
    input_str
}

fn fn_deposit(balance: &mut f64) {
    let input = get_user_input("Please enter deposit amount: ");
    let amount: f64 = match input.trim().parse() {
        Ok(deposit) => deposit,
        Err(_) => {
            println!("Invalid Number");
            return;
        }
    };
    if amount <= 0.0 {
        println!("Amount {} should not be a negative or zero number", amount);
        return;
    }
    *balance += amount;
    println!("After deposit, account balance is {}", balance);

    // ControlFlow::Continue(())
}

// fn fn_withdraw(balance: &mut f64) -> ControlFlow<()> {
fn fn_withdraw(balance: &mut f64) {
    let input = get_user_input("Please enter withdraw amount: ");
    let amount: f64 = match input.trim().parse() {
        Ok(withdraw) => withdraw,
        Err(_) => {
            println!("Invalid Number");
            return;
        }
    };
    if amount <= 0.0 {
        println!("Amount {} should not be a negative or zero number", amount);
        return;
    }
    if amount > *balance {
        println!("Insufficient balance, account balance is {}", balance);
        return;
    }
    *balance -= amount;
    println!("After withdraw, current balance is {}", balance);
}

// fn fn_deposit(balance: &mut f64) -> ControlFlow<()> {
