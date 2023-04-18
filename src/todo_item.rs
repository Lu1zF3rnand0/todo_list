pub struct TodoItem{
    pub description: String,
    pub done: bool,
}

impl TodoItem {
    pub fn new(description: String) -> TodoItem {
        TodoItem {
            description,
            done: false,
        }
    }
}
