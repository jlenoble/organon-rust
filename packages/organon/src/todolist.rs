use crate::{ AtomicU64, fmt, Ordering, Rc, String, ToString, Vec };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TodoId(u64);

impl TodoId {
    pub fn new() -> TodoId {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TodoId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoItem(String);

impl TodoItem {
    pub fn new(item: &str) -> TodoItem {
        TodoItem(item.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Todo {
    pub id: TodoId,
    item: TodoItem,
}

impl Todo {
    pub fn new(item: &str) -> Todo {
        Todo {
            id: TodoId::new(),
            item: TodoItem::new(item),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn create(&mut self, item: &str) -> TodoId {
        let todo = Todo::new(item);
        let id = todo.id;
        self.items.push(todo);
        id
    }

    pub fn delete(&mut self, id: TodoId) -> bool {
        match self.items.iter().position(|todo| { todo.id == id }) {
            Some(pos) => {
                self.items.remove(pos);
                true
            }
            None => false,
        }
    }

    pub fn update(&mut self, id: TodoId, item: &str) -> bool {
        match self.items.iter().position(|todo| { todo.id == id }) {
            Some(pos) => {
                self.items[pos].item = TodoItem(item.to_string());
                true
            }
            None => false,
        }
    }

    pub fn get(&self, id: TodoId) -> Option<Rc<&Todo>> {
        match self.items.iter().position(|todo| { todo.id == id }) {
            Some(pos) => { Some(Rc::new(&self.items[pos])) }
            None => None,
        }
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.items;
        let last = vec.len() - 1;

        for (count, todo) in vec.iter().enumerate() {
            if count != last {
                write!(f, "{}\n", todo.item.0)?;
            }
        }

        write!(f, "{}", vec[last].item.0)
    }
}

#[cfg(test)]
mod tests {
    use super::TodoList;

    #[test]
    fn create_todo() {
        let mut todolist = TodoList::new();

        let id1 = todolist.create("todo1");
        let id2 = todolist.create("todo2");

        assert_eq!(todolist.items[0].id, id1);
        assert_eq!(todolist.items[1].id, id2);

        assert_eq!(todolist.items[0].item.0, "todo1");
        assert_eq!(todolist.items[1].item.0, "todo2");
    }

    #[test]
    fn delete_todo() {
        let mut todolist = TodoList::new();

        todolist.create("todo0");
        let id1 = todolist.create("todo1");
        let id2 = todolist.create("todo2");
        todolist.create("todo3");

        assert_eq!(todolist.items[0].item.0, "todo0");
        assert_eq!(todolist.items[1].item.0, "todo1");
        assert_eq!(todolist.items[2].item.0, "todo2");
        assert_eq!(todolist.items[3].item.0, "todo3");

        assert_eq!(todolist.delete(id1), true);
        assert_eq!(todolist.delete(id1), false);

        assert_eq!(todolist.items[0].item.0, "todo0");
        assert_eq!(todolist.items[1].item.0, "todo2");
        assert_eq!(todolist.items[2].item.0, "todo3");

        assert_eq!(todolist.delete(id2), true);
        assert_eq!(todolist.delete(id2), false);

        assert_eq!(todolist.items[0].item.0, "todo0");
        assert_eq!(todolist.items[1].item.0, "todo3");
    }

    #[test]
    fn update_todo() {
        let mut todolist = TodoList::new();

        todolist.create("todo0");
        let id1 = todolist.create("todo1");
        todolist.create("todo2");
        let id3 = todolist.create("todo3");

        assert_eq!(todolist.items[0].item.0, "todo0");
        assert_eq!(todolist.items[1].item.0, "todo1");
        assert_eq!(todolist.items[2].item.0, "todo2");
        assert_eq!(todolist.items[3].item.0, "todo3");

        todolist.update(id1, "todo10");
        todolist.update(id3, "todo35");

        assert_eq!(todolist.items[0].item.0, "todo0");
        assert_eq!(todolist.items[1].item.0, "todo10");
        assert_eq!(todolist.items[2].item.0, "todo2");
        assert_eq!(todolist.items[3].item.0, "todo35");
    }

    #[test]
    fn get_todo() {
        let mut todolist = TodoList::new();

        let id0 = todolist.create("todo0");
        let id1 = todolist.create("todo1");
        let id2 = todolist.create("todo2");
        let id3 = todolist.create("todo3");

        assert_eq!(todolist.items[0].id, todolist.get(id0).unwrap().id);
        assert_eq!(todolist.items[1].id, todolist.get(id1).unwrap().id);
        assert_eq!(todolist.items[2].id, todolist.get(id2).unwrap().id);
        assert_eq!(todolist.items[3].id, todolist.get(id3).unwrap().id);
    }
}