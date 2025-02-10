use std::io;

fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View task");
        println!("4. Edit task");
        println!("5. Exit ");
        // choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_task(&task_list),
            4 => edit_task(&mut task_list),
            5 => {
                println!("Press any other key to exit!");
                break;
            }
            _ => {
                println!("Wrong input, try again!");
                continue;
            }
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
    let mut task: String = String::new();
    println!("Please enter task details:");
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read input");

    let task = task.trim().to_string();

    if !task.is_empty() {
        task_list.push(task.clone());
        println!("Task added: {}", task);
    } else {
        println!("Task description can't be empty");
    }
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("Task list is empty");
        return;
    }

    let mut task_number: String = String::new();
    println!("Please enter task number to remove:");
    io::stdin()
        .read_line(&mut task_number)
        .expect("Failed to read input");

    let task_number: usize = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if task_number == 0 || task_number > task_list.len() {
        println!("Task number out of range");
        return;
    }

    let task = task_list.get(task_number - 1);

    match task {
        Some(task) => {
            println!("Removing task: {}", task);
            task_list.remove(task_number - 1);
        }
        None => println!("Task not found"),
    }
}

fn view_task(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("Task list is empty");
        return;
    }

    println!("Task list : {:?}", task_list);
}

fn edit_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("Task list is empty");
        return;
    }

    let mut task_number: String = String::new();
    println!("Please enter task number to edit:");
    io::stdin()
        .read_line(&mut task_number)
        .expect("Failed to read input");

    let task_number: usize = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if task_number == 0 || task_number > task_list.len() {
        println!("Task number out of range");
        return;
    }

    if let Some(_) = task_list.get(task_number - 1) {
        let mut new_task: String = String::new();
        println!("Please enter task details:");
        io::stdin()
            .read_line(&mut new_task)
            .expect("Failed to read input");

        let new_task = new_task.trim().to_string();

        if !new_task.is_empty() {
            task_list[task_number - 1] = new_task.clone();
            println!("Task Updated: {}", new_task.clone());
        } else {
            println!("Task description can't be empty");
        }
    } else {
        println!("Task not found");
    }
}
