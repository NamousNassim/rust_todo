use std::io;

fn show_todo(todos: &Vec<String>) {
    for (idx, value) in todos.iter().enumerate() {
        println!("{:?} : {}", idx + 1, value);
    }
}

fn add_todos(todos: &mut Vec<String>) {
    let mut inp = String::new();
    loop {
        println!("Give a todo, type 'stop' if finished with your todos: ");
        inp.clear();
        io::stdin().read_line(&mut inp).expect("Error reading input");

        match inp.trim() {
            "stop" => break,
            _ => todos.push(inp.trim().to_string()),
        }
    }
}
 
fn remove_todo(todo: String, todos: &mut Vec<String>) {
    if let Some(index) = todos.iter().position(|t| t == &todo) {
        todos.remove(index);
        println!(" '{}' removed successfully.", todo);
    } else {
        println!(" '{}' not found.", todo);
    }
}

fn main() {
    let mut todos: Vec<String> = Vec::new();

    add_todos(&mut todos);

    if todos.is_empty() {
        println!("No todos!");
    } else {
        show_todo(&todos);
    }


        let mut todo : String = String::new();
                println!(" todo to remove : "); 

            io::stdin().read_line(&mut todo).expect("error reading todo");
            let todo : String= todo.trim().to_string();
    remove_todo(todo, &mut todos);

            show_todo(&todos)

}
