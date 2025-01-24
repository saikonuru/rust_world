#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    active: bool,
}

fn main() {
    let student1 = Student {
        name: "Sairam".to_string(),
        age: 45,
        active: true,
    };

    let student2 = Student {
        name: String::from("Rajaram"),
        ..student1
    };
    println!("{:#?}", student1);
    println!("{:#?}", student2);
}
