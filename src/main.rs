use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Add a new task")]
    Add { task: String },

    #[structopt(name = "addbatch", about = "Add multiple tasks at once")]
    AddBatch { tasks: Vec<String> },

    #[structopt(name = "list", about = "List all tasks")]
    List,

    #[structopt(name = "complete", about = "Complete a task")]
    Complete { task_index: usize },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TodoList {
    tasks: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
        self.save_to_file();
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get(index) {
            println!("Completing task: {}", task);
            self.tasks.remove(index);
            self.save_to_file();
        } else {
            eprintln!("Error: Invalid task index.");
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            println!("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("  [{}] {}", index, task);
            }
        }
    }

    fn save_to_file(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        fs::write("todo.json", serialized).expect("Unable to write to file");
    }

    fn load_from_file() -> Self {
        if let Ok(contents) = fs::read_to_string("todo.json") {
            serde_json::from_str(&contents).unwrap_or_else(|_| TodoList::new())
        } else {
            TodoList::new()
        }
    }

    fn add_batch_tasks(&mut self, tasks: Vec<String>) {
        for task in tasks {
            self.tasks.push(task);
        }
        self.save_to_file();
    }
}

fn main() {
    let mut todo_list = TodoList::load_from_file();
    let command = Command::from_args();

    match command {
        Command::Add { task } => todo_list.add_task(task),
        Command::AddBatch { tasks } => todo_list.add_batch_tasks(tasks),
        Command::List => todo_list.list_tasks(),
        Command::Complete { task_index } => todo_list.complete_task(task_index),
    }
}
