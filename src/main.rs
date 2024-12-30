const GLOBAL_VARIABLE: u8 = 5;
fn main() {
    let a = 5;
    let mut b = a;
    b = 9;
    println!("a: {}, b: {}", a, b);

    let mut num: u8 = 5;
    num = 6; // warning
    println!("Num: {}", num);

    let str_literal: &str = "Hello World";
    //str_literal = "Hello Sairam"; // not allowed
    println!("String Literal: {}", str_literal);

    let str: String = String::from("Hello World");

    //let str_literal2 = str; // not allowed
    let str_literal2 = str.clone(); //  allowed to fix double free error
                                    // let str_literal2 = str_literal; //  allowed
    println!("String Literal: {}", str_literal2);

    println!("String Literal: {}", str_literal);

    //str="Hello sairam".to_string();
    println!("String: {}", str);

    {
        println!("Global val: {}", GLOBAL_VARIABLE);
    }
    print!("Global val: {}", GLOBAL_VARIABLE);

    let student_info = ("Sairam", 25, 5.8);

    println!("Student Info: {:?}", student_info);
    // let student_info:

    let n = 10;

    if n > 10 {
        println!("n is greater than 10");
    } else if n < 10 {
        println!("n is less than 10");
    }

    let arr = [1, 2, 3, 4, 5];
    println!("Array length: {}", arr.len());

    println!("Print array using for loop");
    for ele in &arr {
        println!("ele: {}", ele);
    }
    //   OR
    // for ele in arr.iter() {
    //     println!("ele: {}", ele);
    // }

    println!("Print array using while loop");
    let mut i = 0;
    while i < arr.len() {
        println!("ele: {}", arr[i]);
        i += 1;
    }
}
