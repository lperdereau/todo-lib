use todolist::todo::Todo;
use todolist::todolist::TodoList;
use todolist::{delete_todolist, list_todolists, save_todolist};

fn main() {
    let mut todolist = TodoList::new(String::from("TodoList1"));
    println!("{:?}", todolist);

    let todo = Todo::new(String::from("Todo1"), false);
    println!("{:?}", todo);

    todolist.add_todo(&todo);

    println!("Todos: {:?}", todolist.todos);

    todolist.todos[0].done = true;
    println!();

    println!("Todos: {:?}", todolist.todos);
    println!();

    println!("TodosLists: {:?}", save_todolist(&todolist));

    todolist.delete_todo(&todo);
    println!("TodosLists: {:?}", list_todolists());
    println!("TodosLists: {:?}", delete_todolist(&todolist));

    let todo2 = Todo { ..todo };
    println!("{:?}", ..todo2);
}
