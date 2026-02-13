use serde::{Deserialize, Serialize};
use std::fs;
use std::env;

fn main() {
    let filename = "tasks.json";
    let mut tasks = load_tasks(filename);

    let args: Vec<String> = env::args().collect();

    let command = parse_command(args);

    match command {
        Some(Command::Add(title)) => {
            add_task(&mut tasks, title);
            save_tasks(&tasks, filename);
        }
        Some(Command::Complete(id)) => {
            complete_task(&mut tasks, id);
            save_tasks(&tasks, filename);
        }
        Some(Command::Delete(id)) => {
            delete_task(&mut tasks, id);
            save_tasks(&tasks, filename);
        }
        Some(Command::List) => {
            list_tasks(&tasks);
        }
        Some(Command::Priority(id, priority)) => {
            set_priority(&mut tasks, id, priority);
            save_tasks(&tasks, filename);
        }
        Some(Command::AddDueDate(id, due_date)) => {
            add_due_date(&mut tasks, id, due_date);
            save_tasks(&tasks, filename);
        }
        None => {
            println!("Usage:");
            println!("todo add \"task\"");
            println!("todo complete 1");
            println!("todo delete 1");
            println!("todo list");
            println!("todo priority 1 high");
            println!("todo end 1 \"2024-12-31\"");
    }
}
}


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
    due_date: Option<String>,
    priority: Option<Priority>,
}

type TaskList = Vec<Task>;

fn load_tasks(filename: &str) -> TaskList {
    match fs::read_to_string(filename) {
        Ok(data) => {
            if data.trim().is_empty() {
                vec![]
            } else {
                serde_json::from_str(&data).unwrap_or(vec![])
            }
        }
        Err(_) => vec![],
    }
}
    

fn save_tasks(tasks: &TaskList, filename: &str) {
    let data = serde_json::to_string_pretty(tasks).expect("Unable to serialize tasks");
    fs::write(filename, data).expect("Unable to write file");
}

fn add_task(tasks: &mut TaskList, title: String) {
    let id = tasks.len() as u32 + 1;
    let task = Task {
        id,
        title,
        completed: false,
        due_date: None,
        priority: None,
    };
    tasks.push(task);
}

fn add_due_date(tasks: &mut TaskList, id: u32, due_date: String) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.due_date = Some(due_date);
    }
}

fn complete_task(tasks: &mut TaskList, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
    }
}

fn delete_task(tasks: &mut TaskList, id: u32) {
    tasks.retain(|t| t.id != id);
}

fn list_tasks(tasks: &TaskList) {
    for task in tasks {
        println!(
            "{} | {} | [{}] | {} | {}",
            task.id,
            task.title,
            if task.completed { "x" } else { " " },
            match &task.due_date {
            Some(date) => format!("Due: {}", date), 
            None => "No due date".to_string(), 
        },
        match &task.priority {
            Some(priority) => format!("Priority: {:?}", priority),
            None => "No priority".to_string(),
        }
        );
    }
}

enum Command { 
    Add(String), 
    Complete(u32), 
    Delete(u32), 
    List, 
    Priority(u32, Priority),
    AddDueDate(u32, String),
}   

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    Low, 
    Medium, 
    High,
}

fn parse_command(args: Vec<String>) -> Option<Command> {
    if args.len() < 2 {
        return None;
    }

    match args[1].as_str() {
        "add" => {
            let title = args.get(2)?.clone();
            Some(Command::Add(title))
        }
        "complete" => {
            let id = args.get(2)?.parse().ok()?;
            Some(Command::Complete(id))
        }
        "delete" => {
            let id = args.get(2)?.parse().ok()?;
            Some(Command::Delete(id))
        }
        "priority" => {
            let id = args.get(2)?.parse().ok()?;
            let priority_str = args.get(3)?.as_str();
            let priority = match priority_str {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => return None,
            };
            Some(Command::Priority(id, priority))
        }
        "end" => {
            let id = args.get(2)?.parse().ok()?;
            let due_date = args.get(3)?.clone();
            Some(Command::AddDueDate(id, due_date))
        }
        "list" => Some(Command::List),
        _ => None,
    }
}



fn set_priority(tasks: &mut TaskList, id: u32, priority: Priority) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.priority = Some(priority);
    }
}