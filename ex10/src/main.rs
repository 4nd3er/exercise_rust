// * Crea un menú de consola para agregar tareas, listarlas y eliminarlas.
use std::io::stdin;

fn std_input (msg: String) -> String {
    let mut input = String::new();
    println!("{}", msg);
    let _ = stdin().read_line(&mut input);
    input.to_string()
}

fn add_todo (todos: &mut Vec<String>, todo: String) {
    todos.push(todo);
}

fn show_todos (todos: &mut Vec<String>) {
    let mut counter = 1;
    for todo in todos {
        println!("{}. {}", counter, todo.to_string());
        counter += 1;
    }
}

fn delete_todo (todos: &mut Vec<String>, mut index: i32) {
    if index != 0 {
        index -= 1;
    }
    todos.remove(index.try_into().unwrap());
}

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        let mut instruction = String::new();

        println!("{:*>20}", "");
        println!("Que quieres hacer?");
        println!("Agregar una tarea (1)");
        println!("Ver las tareas (2)");
        println!("eliminar una tarea (3)");
        println!("{:*>20}", "");
        let _ = stdin().read_line(&mut instruction);

        if instruction.trim() == "q" {
            break;
        }

        let instruction_num: i32 = instruction.trim().parse().expect("numero invalido");
        match instruction_num {
            1 => {
                let new_todo = std_input("Digita la tarea para añadir".to_string());
                add_todo(&mut todos, new_todo);
                println!("Tarea añadida!");
            },
            2 => show_todos(&mut todos),
            3 => {
                show_todos(&mut todos);
                let todo = std_input("Digita la tarea que deseas eliminar".to_string());
                let index_todo: i32 = todo.trim().parse().expect("numero invalido");
                delete_todo(&mut todos, index_todo);
                println!("Tarea eliminada");
            }
            _ => println!("Opción no valida")
        };
    }
}
