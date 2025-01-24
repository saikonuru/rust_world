#![allow(warnings)]
// fn is_palindrome(s: &str) -> bool {
//     let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
//     s == s.chars().rev().collect::<String>()
// }

// fn main() {
//     let test_str = "A man, a plan, a canal, Panama";
//     println!("Is '{}' a palindrome? {}", test_str, is_palindrome(test_str));
// }

// use std::result;

// mod rust_play;

// use std::net::ToSocketAddrs;

use core::num;
use std::result;

pub fn welcome_rust() -> &'static str {
    return "Hello World";
}

pub fn sum(num1: u16, num2: u16) -> u16 {
    return num1 + num2;
}

pub fn celsius_to_fahren(num: f32) -> f32 {
    return (num * 9.0) / 5.0 + 32.0;
}

// pub fn is_even(num: u32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

pub fn is_prime(num: u32) -> bool {
    let i: u32 = 1;
    let mut result = true;
    while i < num / 2 + 1 {
        if num % i == 0 {
            result = false;
            break;
        }
    }
    return result;
}

pub fn factorial(num: u32) -> u32 {
    (1..=num).product()
}

pub fn nth_fib(term: u64) -> u64 {
    if term == 0 {
        return 0; // Base case: F(0) = 0
    }
    if term == 1 {
        return 1; // Base case: F(1) = 1
    }

    let mut prev = 0;
    let mut next = 1;

    for _ in 2..=term {
        let temp = prev + next; // Calculate the next Fibonacci number
        prev = next;
        next = temp;
    }
    next
}

pub fn sum_of_digits(mut num: u64) -> u64 {
    let mut result: u64 = 0;

    let mut temp = num;

    while 0 < num {
        temp = num % 10;
        result += temp;
        num = num / 10;
    }

    return result;
}

// fn is_palindrome(num: u64) -> bool {
//     let mut result: bool = true;

//     let str: Vec<char> = num.to_string().chars().collect();
//     let mut i: u32 = 0;
//     let mut j: u32 = (str.len() as u32) - 1;

//     while j > i {
//         if str[j as usize] != str[i as usize] {
//             result = false;
//             break;
//         }
//         i += 1;
//         j -= 1;
//     }

//     return result;
// }

pub fn is_palindrome(num: u64) -> bool {
    let mut rev = 0;
    let mut digit;
    let mut temp = num;
    while temp > 0 {
        digit = temp % 10; // Extract the last digit
        rev = rev * 10 + digit; // Build the reversed number
        temp /= 10; // Remove the last digit
    }
    num == rev // Check if the original number matches the reversed number
}

/*
fn main() {
    /*
       welcome_rust();
       let result = sum(10, 18);
       println!("Sum : {}", result);

       println!("Celsius to Fahrenheit : {}", celsius_to_fahren(32.0));

       let num = 8;
       let result: bool = is_even(9);

       if result {
           println!("{} is even", num);
       } else {
           println!("{} is odd", num);
       }

       let num = 11;
       let result: bool = is_prime(9);

       if result {
           println!("{} is a prime number", num);
       } else {
           println!("{} is not a prime number", num);
       }
    */
    println!("result {}", is_palindrome(1221));
}

*/

fn is_even(num: u32) -> bool {
    return if num % 2 == 0 { true } else { false };
}

fn print_str(str_passed: String) {
    println!("{}", str_passed);
}

// does't allows the str to re use
// fn cal_length(str_passed: String) -> usize {
//     str_passed.len()
// }

fn cal_length(str_passed: String) -> (String, usize) {
    let len = str_passed.len();
    (str_passed, len)
}

// temperately passing ownership - allows to re-use
// fn cal_length_ref(str_passed: &String) -> usize {
//     str_passed.len()
// }

// passing by value will not affect the original value
pub fn print_x(mut x: i32) {
    x = 19;
    println!("{}", x);
}

fn append_str(str: &mut String) {
    str.push_str(" World");
}

pub fn append_string(mut str: String) -> String {
    str.push_str(" World!");
    str
}

// Dereferencing

fn cal_length_ref2(str_passed: &String) -> usize {
    (*str_passed).len()
}

fn shadowing() {
    println!("shadowing demo");
    let s1 = String::from("Hello");
    let s1 = s1.len();
    println!("{}", s1);
}

mod math_lib;
use math_lib::maths::*;

fn main() {
    let str = String::from("hello");
    // print_str(str);

    //let len = cal_length(str); // does't allows the str to re use

    let (str, len) = cal_length(str);
    println!("Length of {} is {}", str, len);

    // passing reference Borrowing the value of str to cal_length_ref
    //function and not taking ownership of it so that it can be
    // used again in the main function
    // println!("Reference Length of {} is {}", str, cal_length_ref(&str));

    let mut s1: String = String::from("Hello");
    let r1: &String = &s1;
    let r2: &String = &s1;
    let r3: &String = &s1;

    // allows any number of copies of reference
    println!("s1:{} r1: {}, r2: {} r3: {}", s1, r1, r2, r3);

    let x = 5;
    // passing by value will not affect the original value
    print_x(x);
    println!("{}", x);
    // print_str(str);// not allowed twice
    // println!("{}", str); //not allowed

    // Change the value of the string in the calling function passing by reference
    let mut s2 = String::from("Hello");
    append_str(&mut s2);
    println!("Appended string: {}", s2);

    // Change the value of the string in the calling function passing by value
    // let mut s3 = String::from("Hello");
    println!("New Appended string: {}", append_string(s2));
    // println!("New Appended string: {}", s2); // not allowed as s2 ownership is changed

    // Dereferencing
    println!("Dereferencing");
    // mutable variable and reference
    // let x = 5;
    // let y = &x;
    // let z = &y;
    //x = 10; // Not allows as the variable and its reference are immutable

    //    println!("x: {}, y: {}, z: {}", x, y, z);
    // println!("x: {}, y: {}, z: {}", x, *y, z);

    let mut x = 5;
    let mut y = &mut x;
    *y = 10;
    let z = &mut y;
    **z = 15;
    println!("x: {}", x);
    // can be accessed as below as well
    // println!("x: {}", *y);
    //println!("x: {}", **z);
    //println!("y: {}", *y); // Not allowed as borrowing the value of y

    // let x = String::from("Hello");
    // let y = &x;
    // let z = y;
    // println!("x: {}, y: {}, z: {}", x, y, z);
    // println!("x: {}, y: {}, z: {}", x, *y, *z);

    // ! Better comment test
    // * Better comment test
    // ** Better comment test
    // ? Better comment test
    // *** Better comment test
    // * TODO: Better comment test
    println!("s1 len : {}", cal_length_ref2(&s1));
    let emoji = "❤️👍🛒";
    let kanji = "漢字";
    let emoji_family = "👨‍👩‍👧‍  👦";

    shadowing();
    // let result = maths::add(10, 20);

    println!("Addition : {}", add(10, 20));
    println!("Subtraction : {}", sub(100, 20));

    let x: u32 = 22;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    match x {
        n if is_even(n) => println!("Even"),
        n if !is_even(n) => println!("Odd"),
        _ => println!("Other"), // *! default case is necessary
    }

    guessNumber();
    let a: u32 = getNumber("Enter the first number: ");
    let b: u32 = getNumber("Enter the second number: ");

    println!("Addition : {}", add(a, b));
    println!("Subtraction : {}", sub(a, b));
    match div(a, b) {
        Some(result) => println!("Division : {}", result),
        None => println!("Division by zero is not allowed"),
    }
    println!("Multiplication : {}", mul(a, b));
    println!("Square root of {} : {}", a, sqrt(a));
}

use std::io;

fn getNumber(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Not valid input");
        //let num: u32 = input.trim().parse().expect("Not a number");
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Try again!"),
        }
    }
}

use rand::Rng;
use std::cmp::Ordering;
fn guessNumber() {
    let random_number = rand::thread_rng().gen_range(1..=100) as u32;

    loop {
        let mut input = String::new();
        println!("Guess a number between 1 and 100: ");
        io::stdin().read_line(&mut input).expect("Not valid input");

        let guess_number: u32 = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again!");
                continue;
            }
        };

        // match num {
        //     num if num == guess => {
        //         println!("you own!");
        //         break;
        //     }
        //     num if num > guess => println!("Too high"),
        //     num if num < guess => println!("Too low"),
        //     _ => println!("game over"),
        // }
        println!("Random Number {}", random_number);
        // ** using cmp method
        match guess_number.cmp(&random_number) {
            Ordering::Less => println!("Guess number is too small!"),
            Ordering::Greater => println!("Guess number is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    }
    return b - a;
}

fn div(a: u32, b: u32) -> Option<f64> {
    if b == 0 {
        return None;
    }
    return Some((a as f64) / (b as f64));
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}

fn sqrt(a: u32) -> f64 {
    return (a as f64).sqrt();
}

// fn div(a: u32, b: u32) -> Option<f64> {
//     if b == 0 {
//         None // Return None if division by zero
//     } else {
//         Some((a as f64) / (b as f64)) // Return the result wrapped in Some
//     }
// }
