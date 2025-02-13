fn main() {
    let x = 10;
    let y = 20;

    let res = largest(&x, &y);

    println!("{}", res);
}

fn largest<'lt>(a: &'lt i32, b: &'lt i32) -> &'lt i32 {
    if a > b {
        a
    } else {
        b
    }
}
