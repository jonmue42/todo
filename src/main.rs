pub mod todolist;
pub mod tui;

fn main() {
    let todo1 = todolist::Todo {
        text: String::from("This is todo1"),
        done: false,
    };

    println!("{:?}", todo1);

    let mut todolist1 = todolist::Todolist {
        list: Vec::new(),
    };
   
    println!("{:?}", todolist1.list);

    todolist1.add_todo(todo1);

    println!("{:?}", todolist1.list);

    todolist1.list[0].mark_done();

    println!("{:?}", todolist1.list);

    tui::start_tui();

}
