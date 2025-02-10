// Generic type

struct Pair<T, U> {
    value1: T,
    value2: U,
}

// ! Implementation using reference
impl<T, U> Pair<T, U> {
    fn get_value1(&self) -> &T {
        &self.value1
    }

    fn get_value2(&self) -> &U {
        &self.value2
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            value1: self.value2,
            value2: self.value1,
        }
    }
}

// ! Another implementation , clone is expensive operation
// impl<T: Clone, U: Clone> Pair<T, U> {
//     fn get_value1(&self) -> T {
//         self.value1.clone()
//     }

//     fn get_value2(&self) -> U {
//         self.value2.clone()
//     }

//     fn swap(self) -> Pair<U, T> {
//         Pair {
//             value1: self.value2,
//             value2: self.value1,
//         }
//     }
// }

fn main() {
    let pair = Pair {
        value1: 100,
        value2: "hello",
    };

    println!("Value 1: {}", pair.get_value1());
    println!("Value 2: {}", pair.get_value2());
    let swapped_pair = pair.swap();
    println!("Value 1: {}", swapped_pair.get_value1());
    println!("Value 2: {}", swapped_pair.get_value2());
}
