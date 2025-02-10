// // Just to access
// fn main() {
//     let arr = [String::from("hello"), String::from("world")];

//     for item in arr.iter() {
//         println!("{}", item);
//     }

//     println!("{:?}", arr); // ! allowed
//     for item in arr {
//         println!("{}", item);
//     }

//     // println!("{:?}", arr); // !Not allowed
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        //  Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associative type

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

// To update
fn main() {
    let mut arr = [String::from("hello"), String::from("world")];

    for item in arr.iter_mut() {
        *item = format!("{}!", item);
        // or you can use item.push_str("!");
        println!("{}", item);
    }

    println!("{:?}", arr); // ! Allowed
    for item in arr {
        println!("{}", item);
    }
    // println!("{:?}", arr); // !Not allowed

    let arr = [1, 2, 3];
    let mut iterators = arr.iter(); // returns reference
    assert_eq!(Some(&1), iterators.next());

    if Some(&2) == iterators.next() {
        println!("worked");
    }

    let mut iterators = arr.into_iter(); // returns the value
    assert_eq!(Some(1), iterators.next());

    let mut counter = Counter::new();

    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}
