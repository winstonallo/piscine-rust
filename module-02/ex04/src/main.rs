use ftkit;

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { 
            todos: Vec::new(),
            dones: Vec::new()
        }
    }

    fn display(&self) {
        let todos = self.todos.clone();
        for i in 0..todos.len() {
            println!("{} [ ] {}", i, todos[i]);
        }
        let dones = self.dones.clone();
        for done in dones {
            println!("  [x] {}", done);
        }
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }

    fn done(&mut self, index: usize) {
        if index >= self.todos.len() {
            println!("{} out of range for self.todos (size: {})", index, self.todos.len());
            return;
        } else {
            let done_task = self.todos.remove(index);
            self.dones.push(done_task);
        }
    }

    fn purge(&mut self) {
        self.dones.clear();
    }
}

enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

impl Command {
    fn prompt() -> Self {
        loop {
            let input = ftkit::read_line();

            if input.is_empty() || input.trim() == "QUIT" {
                break Self::Quit;
            } else if let Some(arg) = input.strip_prefix("TODO ") {
                break Self::Todo(arg.trim().to_string());
            } else if let Some(idx) = input.strip_prefix("DONE ") {
                if let Ok(idx) = idx.trim().parse() {
                    break Self::Done(idx);
                }
            } else if input.trim() == "PURGE" {
                break Self::Purge;
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        todo_list.display();
        println!();
        match Command::prompt() {
            Command::Todo(todo) => todo_list.add(todo),
            Command::Done(idx) => todo_list.done(idx),
            Command::Purge => todo_list.purge(),
            Command::Quit => break,
        }
    }
}