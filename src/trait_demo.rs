struct Student {
    name: String,
}

trait Name {
    fn change_name(&mut self, new_name: String);
}

impl Name for Student {
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

// fn remove_whitespace(input: &mut String) {
//     let words = input.split_whitespace();
//     let _vec: Vec<&str> = words.collect();
//     let _joined_words = _vec.join("");
//     println!("{:?}", _joined_words);
// }

// ** With one variable
fn remove_whitespace(input: &mut String) {
    let joined_words: String = input.split_whitespace().collect::<Vec<&str>>().join("");
    println!("{:?}", joined_words);
}

// fn reverse_str(input: &mut String) {
//     let trimmed_input = input.trim();
//     let chars = trimmed_input.chars();
//     let rev_char = chars.rev();
//     let rev_str: String = rev_char.collect();
//     println!("{:?}", rev_str);
// }

// ** With one variable
fn reverse_str(input: &mut String) {
    println!("{:?}", input.trim().chars().rev().collect::<String>());
}

fn is_palindrome(input: &mut String) {
    let rev: String = input.trim().chars().rev().collect();

    if rev.to_string() == input.trim() {
        println!("Yes");
    } else {
        println!("no");
    }
}

use std::io;
fn main() {
    // let mut student = Student {
    //     name: "sairam".to_string(),
    // };
    // println!("Student name {}", student.name);
    // student.change_name("Ram Sai".to_string());
    // println!("Student name {}", student.name);

    println!("Enter a string");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed inputs");

    remove_whitespace(&mut input);
    reverse_str(&mut input);

    is_palindrome(&mut input);
}
