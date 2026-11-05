// * Crea un gestor de tareas que almacene su información en un archivo JSON.
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, stdin},
};

#[derive(Serialize, Deserialize, Debug)]
enum State {
    Pending,
    Completed,
}
struct Constraints;

impl Constraints {
    const DIR_FILE: &str = "todo.json";
    const TASK_KEYS: [&str; 4] = ["nombre", "descripcion", "fecha", "estado"];
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    name: String,
    description: String,
    date: String,
    state: State,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    tasks: Vec<Task>,
}

fn find_file() -> Result<Tasks, Box<dyn Error>> {
    // let file = File::options().append(true).open("todo.json")?;
    let file = File::open(Constraints::DIR_FILE)?;
    let reader = BufReader::new(&file);
    let json: Tasks = serde_json::from_reader(reader).unwrap();
    Ok(json)
}

fn p_a_i(str: &str) -> String {
    let mut input = String::new();
    println!("{}", str);
    let _ = stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_string()
}

fn create_task(id_task: i32) -> Task {
    let mut values: Vec<String> = Vec::with_capacity(Constraints::TASK_KEYS.len());

    for key in Constraints::TASK_KEYS {
        let mut string = String::from("Digita el ");
        string.push_str(key);
        let input = p_a_i(string.as_str());
        values.push(input);
    }

    let new_task = Task {
        id: id_task,
        name: values[0].to_string(),
        description: values[1].to_string(),
        date: values[2].to_string(),
        state: State::Pending,
    };
    new_task
}

fn add_task(json: &mut Tasks, new_task: Task) {
    json.tasks.push(new_task);
}

fn search_task<'a>(json: &'a mut Tasks, id: i32) -> Result<Vec<&'a mut Task>, String> {
    if id == 0 || id > json.tasks.len() as i32 {
        return Err("No se encontro la tarea".to_string());
    }
    Ok(json.tasks.iter_mut().filter(|task| task.id == id).collect())
}

fn edit_task(task: &mut Vec<&mut Task>) {
    println!("Que campo quieres editar?");
    for i in 0..Constraints::TASK_KEYS.len() {
        println!("({}) {}", i + 1, Constraints::TASK_KEYS[i]);
    }
    let input = p_a_i("");
    let instruction: i32 = input.parse().unwrap_or_default();
    match instruction {
        1 => {
            let input = p_a_i("Digite el nuevo valor");
            task[0].name = input.trim().to_string();
        }
        2 => {
            let input = p_a_i("Digite el nuevo valor");
            task[0].description = input.trim().to_string();
        }
        3 => {
            let input = p_a_i("Digite el nuevo valor");
            task[0].date = input.trim().to_string();
        }
        4 => {
            let _ = p_a_i("Digite el nuevo valor");
            task[0].state = State::Completed;
        }
        _ => println!("Instruccion no valida"),
    }
}

fn update_file(json: &Tasks) -> Result<(), Box<dyn Error>> {
    let new_file = File::create(Constraints::DIR_FILE).unwrap();
    serde_json::to_writer_pretty(new_file, &json)?;
    Ok(())
}

fn main() {
    match find_file() {
        Ok(mut json) => {
            println!("{:?}", json.tasks);
            loop {
                let mut input = String::new();
                println!("Que quieres hacer?");
                println!("(1) Crear");
                println!("(2) Buscar");
                println!("(3) Actualizar");
                println!("(4) Eliminar");
                let _ = stdin().read_line(&mut input).unwrap_or_default();
                input = input.trim().to_string();

                if input == "q" {
                    break;
                }

                let instruction: i32 = input.parse().unwrap_or_default();
                match instruction {
                    1 => {
                        // ? Añadir
                        let new_task = create_task(json.tasks.len() as i32 + 1);
                        add_task(&mut json, new_task);
                        match update_file(&json) {
                            Ok(_) => println!("Archivo actualizado!"),
                            Err(error) => println!("error: {}", error),
                        }
                    }
                    2 => {
                        // ? Buscar
                        let input = p_a_i("id");
                        let id: i32 = input.parse().unwrap_or(0);
                        match search_task(&mut json, id) {
                            Ok(task) => {
                                println!("{}", task[0].name);
                                println!("{}", task[0].description);
                            }
                            Err(error) => println!("{}", error),
                        }
                    }
                    3 => {
                        // ? Editar
                        let input = p_a_i("Digita el id");
                        let id: i32 = input.parse().unwrap_or(0);
                        match search_task(&mut json, id) {
                            Ok(mut task) => {
                                edit_task(&mut task);
                                
                                match update_file(&json) {
                                    Ok(_) => println!("Archivo actualizado!"),
                                    Err(error) => println!("error: {}", error),
                                }
                            }
                            Err(error) => println!("{}", error),
                        }
                    }
                    4 => {
                        // ? Eliminar
                        println!("Eliminar");
                    }
                    5 => {
                        for task in &json.tasks {
                            println!("{:?}", task);
                        }
                    }
                    _ => println!("Instruccion no valida"),
                }
                println!();
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
