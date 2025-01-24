use std::{io, ops::ControlFlow};

fn main() {
    let mut balance: f64 = 0.0;
    loop {
        println!("Please choose an option");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Balance");
        println!("Any other key exit");
        let mut input_str = String::new();

        println!("Please your choice: ");
        io::stdin()
            .read_line(&mut input_str)
            .expect("Invalid Input");
        let input: u32 = match input_str.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };

        match input {
            1 => {
                if let ControlFlow::Break(_) = fn_deposit(&mut balance) {
                    continue;
                }
            }
            2 => {
                if let ControlFlow::Break(_) = fn_withdraw(&mut balance) {
                    continue;
                }
            }
            3 => println!("Balance:{}", balance),
            _ => {
                println!("Thank you!");
                break;
            }
        }
    }
}

fn fn_withdraw(balance: &mut f64) -> ControlFlow<()> {
    let mut withdraw_input = String::new();
    println!("Please withdraw amount: ");
    io::stdin()
        .read_line(&mut withdraw_input)
        .expect("Invalid Input");
    let withdraw: f64 = match withdraw_input.trim().parse() {
        Ok(withdraw) => withdraw,
        Err(_) => {
            println!("Invalid Number");
            return ControlFlow::Break(());
        }
    };
    if withdraw > *balance {
        println!("Insufficient balance, account balance is {}", balance);
        return ControlFlow::Break(());
    }
    *balance -= withdraw;
    println!("After withdraw, current balance is {}", balance);

    ControlFlow::Continue(())
}

fn fn_deposit(balance: &mut f64) -> ControlFlow<()> {
    let mut deposite_input = String::new();
    println!("Please deposit amount: ");
    io::stdin()
        .read_line(&mut deposite_input)
        .expect("Invalid Input");
    let deposit: f64 = match deposite_input.trim().parse() {
        Ok(deposit) => deposit,
        Err(_) => {
            println!("Invalid Number");
            return ControlFlow::Break(());
        }
    };
    if deposit <= 0.0 {
        println!("Deposit should not bea negative or zero number");
        return ControlFlow::Break(());
    }
    *balance += deposit;
    println!("After Deposit account balance is {}", balance);

    ControlFlow::Continue(())
}
