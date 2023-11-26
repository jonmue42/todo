// struct for the individual Todos
#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

// struct for todolist consisting of only vec of Todo structs
#[derive(Debug)]
pub struct Todolist { 
    pub list: Vec<Todo>
}

impl Todolist {
    pub fn add_todo(&mut self, new_todo: Todo) {
        self.list.push(new_todo);
    }
}

