use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please enter a number: ");
        io::stdin().read_line(&mut input).expect("Invalid Input");
        let number: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };
        binary_converter(number);
        hex_converter(number);
        break;
    }
}
//1 2 3 4 5 6 .. 10 -> (1..=10)
//10 9 8 7 6 5 ... 1 -> (1..=10).rev()

// fn to_hex(number: u16) {
//     println!(
//         "Number {} in hexadecimal : {}",
//         number,
//         format!("{:b}", number)
//     );
// }

// fn to_binary(number: u16) {
//     println!("Number {} in binary : {}", number, format!("{:b}", number));
// }

fn hex_converter(mut decimal_val: u32) {
    let mut hex_str: String = String::new();
    while decimal_val > 0 {
        let reminder: u8 = (decimal_val % 16) as u8;
        if reminder > 9 {
            hex_str.insert(0, (reminder + 55) as char);
        } else {
            hex_str.insert(0, (reminder + 48) as char);
        }
        decimal_val = decimal_val / 16;
    }
    println!("Hex number: {}", hex_str);
}

fn binary_converter(mut decimal_val: u32) {
    let mut binary_str: String = String::new();
    while decimal_val > 0 {
        let reminder: u8 = (decimal_val % 2) as u8;
        binary_str.insert(0, (reminder + 48) as char);
        //print!("{}", reminder);
        decimal_val = decimal_val / 2;
    }

    println!("Binary number: {}", binary_str);
}
