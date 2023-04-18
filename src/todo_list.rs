use crate::TodoItem::TodoItem;

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList{items: Vec::new()}
    }

    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn mark_done(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index){
            item.done = true;
        }
    }

    pub fn print(&self) {
        println!("TODO List:")
        for(index, item) in self.items.iter().enumerate() {
            println!(
                "{}. [{}] {}",
                index, + 1,
                if item.done { "X" } else { " " },
                item.description
            );
        }
    }
}
