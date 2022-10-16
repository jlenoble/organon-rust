use organon::todolist::TodoList;

fn main() {
    let mut todolist = TodoList::new();

    todolist.create("todo0");
    todolist.create("todo1");
    todolist.create("todo2");
    todolist.create("todo3");

    println!("{}", todolist);
}