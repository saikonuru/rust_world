macro_rules! say_hello {
    () => {
        println!("Hello World");
    };
}

macro_rules! repeat_message {
    ($msg:expr,$times:expr) => {
        for _ in 0..$times {
            println!("{}", $msg);
        }
    };
}

use serde::Serialize;
#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    say_hello!();
    repeat_message!("Rust is awesome!", 3); //  ! indicates macro

    let user = User {
        name: "Sairam".to_string(),
        age: 30,
    };

    println!("{}", serde_json::to_string(&user).unwrap());

    let increment = |x: i32| x + 1;
    println!("{}", increment(5));

    let add = |x, y| x + y;
    println!("{}", add(4, 5));

    let mut counter = 100;
    let mut inc_counter = || {
        counter += 1;
        println!("{}", counter);
    };

    inc_counter();
    inc_counter();
    inc_counter();

    let values: Vec<i32> = vec![10, 11, 12, 13, 14, 15];
    let even_vector: Vec<i32> = values.clone().into_iter().filter(|x| x % 2 == 0).collect();
    let odd_vector = values
        .clone()
        .into_iter()
        .filter(|x| x % 2 != 0)
        .collect::<Vec<i32>>();

    println!("Even Vector{:?}", even_vector);
    println!("Odd Vector{:?}", odd_vector);

    // ownership closure

    let str = String::from("hello sai");

    let consume = || str;
    let y = consume();
    // let z = consume();

    println!("{}", y);
}
