use once_cell::sync::Lazy;
use std::sync::Mutex;

use self::todolist::TodoList;

static TODOLIST_ARRAY: Lazy<Mutex<Vec<TodoList>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn save_todolist(todolist: &self::todolist::TodoList) -> Vec<self::todolist::TodoList> {
    TODOLIST_ARRAY.lock().unwrap().push(todolist.clone());
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

pub fn delete_todolist(todolist: &self::todolist::TodoList) -> Vec<self::todolist::TodoList> {
    let index: usize = TODOLIST_ARRAY
        .lock()
        .unwrap()
        .iter()
        .position(|x| x.id == todolist.id)
        .unwrap();
    TODOLIST_ARRAY.lock().unwrap().remove(index);
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

pub fn list_todolists() -> Vec<self::todolist::TodoList> {
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

pub mod todo {
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct Todo {
        pub id: Uuid,
        pub comment: String,
        pub done: bool,
    }

    impl Todo {
        pub fn new(comment: String, done: bool) -> Self {
            Self {
                id: Uuid::new_v4(),
                comment,
                done,
            }
        }
    }
}

pub mod todolist {
    use super::todo::Todo;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct TodoList {
        pub id: Uuid,
        pub title: String,
        pub todos: Vec<Todo>,
    }

    impl TodoList {
        pub fn new(title: String) -> Self {
            Self {
                id: Uuid::new_v4(),
                title,
                todos: vec![],
            }
        }

        pub fn add_todo(&mut self, todo: &Todo) -> &mut Self {
            self.todos.push(todo.clone());
            self
        }

        pub fn delete_todo(&mut self, todo: &Todo) -> &mut Self {
            let index: usize = self.todos.iter().position(|x| x.id == todo.id).unwrap();
            self.todos.remove(index);
            self
        }
    }
}
