use std::io;


fn main() {
    let mut saved_todos: Vec<String> = Vec::new();

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input!");

        let mut user_input = user_input.split_whitespace();

        let cmd: &str = match user_input.next() {
            Some(cmd) => cmd,
            None => continue,
        };

        match cmd {
            "add" => {
                let mut result = String::new();

                for part in user_input {
                    result += " ";
                    result += part;
                }

                saved_todos.push(result);
            }
            "list" => {
                for (index, todo) in saved_todos.iter().enumerate() {
                    println!("{}: {}", index + 1, todo.trim());
                }
            }
            "exit" => break,
            _ => continue
        }
    }
}
