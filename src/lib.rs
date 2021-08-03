use once_cell::sync::Lazy;
use std::sync::Mutex;

static TODOLIST_ARRAY: Lazy<Mutex<Vec<TodoList>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn save_todolist(todolist: &self::TodoList) -> Vec<self::TodoList> {
    TODOLIST_ARRAY.lock().unwrap().push(todolist.clone());
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

pub fn delete_todolist(todolist: &self::TodoList) -> Vec<self::TodoList> {
    let index: usize = TODOLIST_ARRAY
        .lock()
        .unwrap()
        .iter()
        .position(|x| x.id == todolist.id)
        .unwrap();
    TODOLIST_ARRAY.lock().unwrap().remove(index);
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

pub fn list_todolists() -> Vec<self::TodoList> {
    return TODOLIST_ARRAY.lock().unwrap().to_vec();
}

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

    pub fn delete_todo(&mut self, todo: &Todo) -> Todo {
        let index: usize = self.todos.iter().position(|x| x.id == todo.id).unwrap();
        self.todos.remove(index)
    }
}
