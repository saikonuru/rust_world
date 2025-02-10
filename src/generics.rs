// Generic type
#[warn(dead_code)]
struct Container<T> {
    value: T,
}

impl<T: Clone> Container<T> {
    fn new(new_val: T) -> Self {
        Self { value: new_val }
    }
    fn set(&mut self, new_val: T) {
        self.value = new_val;
    }
    fn get(&self) -> T {
        self.value.clone()
    }
}

fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value is {}", value);
}

fn main() {
    let num = 23;
    print_value(num);
    let dec: f32 = 18.19;
    print_value(dec);
    let status: bool = true;
    print_value(status);

    let mut container = Container::new(100);

    println!("Value :{}", container.get());
    container.set(200);
    // container.set(12.34); // not allowed

    println!("New Value :{}", container.get());
}
