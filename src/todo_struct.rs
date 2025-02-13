//To Do list - add task, remove task, view task, exit
use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: u8) -> Self {
        Self {
            description,
            priority,
            completed: false,
        }
    }
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View task");
        println!("4. View Completed tasks");
        println!("5. View Pending tasks");
        println!("6. Mark Complete");
        println!("7. Change Priority");
        println!("8. Exit");

        io::stdin().read_line(&mut choice).expect("Invalid input");
        let choice: i32 = choice.trim().parse().expect("Invalid number");

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_task(&task_list),
            4 => view_completed_task(&task_list),
            5 => view_pending_task(&task_list),
            6 => mark_complete(&mut task_list),
            7 => change_priority(&mut task_list),
            8 => {
                println!("Exiting....");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    let mut description = String::new();

    println!("Please enter a description for the task:");

    io::stdin()
        .read_line(&mut description)
        .expect("Invalid input");

    let description = description.trim().to_string();

    let mut priority = String::new();

    println!("Please enter a priority for the task:");

    io::stdin().read_line(&mut priority).expect("Invalid input");

    let priority: u8 = priority.trim().parse().expect("Invalid input");

    if !description.is_empty() && (1..=5).contains(&priority) {
        task_list.push(Task::new(description, priority));
    } else {
        println!("Description or priority is not correct");
    }
}

fn remove_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Please enter the task no. which you want to remove");

    view_task(task_list);

    let mut task_number = String::new();

    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");

    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > task_list.len() {
                println!("Wrong task number");

                return;
            }

            task_list.remove(task_number - 1);

            println!("Task is removed");
        }

        Err(error) => {
            println!("{}", error);

            return;
        }
    };
}

fn view_task(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Task list: {:?}", task_list);
}

fn view_completed_task(task_list: &Vec<Task>) {
    let completed_tasks: Vec<&Task> = task_list.iter().filter(|task| task.completed).collect();

    if completed_tasks.is_empty() {
        println!("No completed tasks are found");
        return;
    }

    for task in completed_tasks {
        println!("{:?}", task);
    }
}

fn view_pending_task(task_list: &Vec<Task>) {
    let pending_tasks: Vec<&Task> = task_list.iter().filter(|task| !task.completed).collect();

    if pending_tasks.is_empty() {
        println!("No pending tasks are found");
        return;
    }

    for task in pending_tasks {
        println!("{:?}", task);
    }
}

fn mark_complete(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Please enter the task no. which you want to mark as completed");

    view_task(task_list);

    let mut task_number = String::new();

    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");

    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > task_list.len() {
                println!("Wrong task number");
                return;
            }
            if let Some(task) = task_list.get_mut(task_number - 1) {
                task.completed = true;
                println!("Task is marked as completed");
            }
        }

        Err(error) => {
            println!("{}", error);

            return;
        }
    };
}

fn change_priority(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }

    println!("Please enter the task no. which you want change priority");

    view_task(task_list);

    let mut task_number = String::new();

    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");

    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > task_list.len() {
                println!("Wrong task number");
                return;
            }

            if let Some(task) = task_list.get_mut(task_number - 1) {
                let mut priority = String::new();
                println!("Enter task priority: ");
                io::stdin().read_line(&mut priority).expect("Invalid input");
                let priority: u8 = priority.trim().parse().expect("invalid");
                task.priority = priority;
                println!("Task priority changed");
            }
        }

        Err(error) => {
            println!("{}", error);
            return;
        }
    };
}

// fn edit_task(task_list: &mut Vec<String>) {

//     if task_list.is_empty() {

//         println!("no task are found");

//            }

//     println!("Please enter the task no. which you want to edit:");

//     view_task(&task_list);

//     let mut task_num = String::new();

//     io::stdin().read_line(&mut task_num).expect("invalid input");

//     let task_num: usize = task_num.trim().parse().expect("invalid");

//     println!("type updated task");

//     let mut new_task = String::new();

//     io::stdin()

//         .read_line(&mut new_task)

//         .expect("something went wrong");

//     task_list[task_num - 1] = new_task.trim().to_string();

// }
