use todolist::{save_todolist, list_todolists};
use todolist::todo::Todo;
use todolist::todolist::TodoList;

fn main() {
    let mut todolist = TodoList::new(String::from("TodoList1"));
    println!("{:?}", todolist);

    let todo = Todo::new(String::from("Todo1"), false);
    println!("{:?}", todo);

    todolist.add_todo(todo);

    println!("Todos: {:?}", todolist.todos);
    
    println!("");

    println!("TodosLists: {:?}", save_todolist(todolist));
    println!("TodosLists: {:?}", list_todolists());
}
