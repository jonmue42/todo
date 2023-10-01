pub mod todolist;

fn main() {
    let mut todolist1 = todolist::Todolist {todos: Vec::new() };
    
    let mut todo1 = todolist::Todo {text: String::from("Test"), done: false, };

    let mut todo2 = todolist::Todo {text: String::from("Test2"), done: false, };
    
    todolist1.add(&mut todo1);
    println!("{:?}", todolist1);

    todolist1.add(& todo2);
    println!("{:?}", todolist1);

    todo2.mark_done();
    println!("{:?}", todolist1);

    
}
