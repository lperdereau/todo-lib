#[cfg(test)]
mod tests {

    use todolist::Todo;
    use todolist::TodoList;
    use todolist::{delete_todolist, list_todolists, save_todolist};

    /* Add a todolist
        acceptation criterias:
        - the todolist is present in the list of todolists
    */
    #[test]
    fn add_a_todolist() {
        let todolist = TodoList::new(String::from("todolist"));
        let todolists = save_todolist(&todolist);
        assert_eq!(todolists[0], todolist);
    }

    /* Delete a todolist
        acceptation criterias:
        - all the subsequent todos must be deleted
    */
    #[test]
    fn delete_a_todolist() {
        let todolist = TodoList::new(String::from("todolist"));
        let saved_todolists = save_todolist(&todolist);
        let todolists = delete_todolist(&todolist);

        assert_eq!(todolists.len(), saved_todolists.len() - 1);
    }

    /* List all todolists
        acceptation criteria:
        - returns all todolists
    */
    #[test]
    fn list_all_todolists() {
        let todolist = TodoList::new(String::from("todolist"));
        let saved_todolists = save_todolist(&todolist);
        let list_todolists = list_todolists();

        assert_eq!(saved_todolists, list_todolists);
    }

    /* Add a todo to a list
        acceptation criteria:
        - the todo is present in the todolist
        - the todolist doesn't have to be ordered
    */
    #[test]
    fn add_a_todo_to_a_list() {
        let mut todolist = TodoList::new(String::from("todolist"));
        let todo = Todo::new(String::from("new todo"), true);
        let update_todolist = todolist.add_todo(&todo);

        assert_eq!(update_todolist.todos[0], todo);
    }

    /* Delete a todo to a list
        acceptation criteria:
        - the todo is not present in the todolist anymore but the others are
    */
    #[test]
    fn delete_a_todo_to_a_list() {
        let mut todolist = TodoList::new(String::from("todolist"));
        let todo = Todo::new(String::from("new todo"), true);
        todolist.add_todo(&todo);
        let removed_todo = todolist.delete_todo(&todo);

        assert_eq!(todolist.todos.len(), 0);
        assert_eq!(removed_todo, todo);
    }
}
