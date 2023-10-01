pub fn show_todos(status: &Todo) {
    println!("todo list");
    println!("Status {}", status.done);
}

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
pub struct Todolist<'a> { 
    pub todos: Vec<&'a Todo>, 
}

impl<'a> Todolist<'a> {
    pub fn add(&mut self, todo: &'a Todo) {
       self.todos.push(todo);
    }

    pub fn mark_done(&mut self, elem: usize) {
       self.todos[elem].mark_done();
    }
}
