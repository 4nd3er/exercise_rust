// * Crea un gestor de tareas que almacene su información en un archivo JSON.
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, stdin},
};

#[derive(Serialize, Deserialize, Debug)]
enum State {
    Completed,
    Finished,
}
struct DirFile;

impl DirFile {
    const DIR: &str = "todo.json";
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
    let file = File::open(DirFile::DIR)?;
    let reader = BufReader::new(&file);
    let json: Tasks = serde_json::from_reader(reader).unwrap();
    Ok(json)
}

fn p_a_i(str: &str) -> String {
    let mut input = String::new();
    println!("Digita el {}", str);
    let _ = stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_string()
}

fn create_task(id_task: i32) -> Task {
    let task_strs = vec!["nombre", "descripcion", "fecha", "estado"];
    let mut values: Vec<String> = Vec::with_capacity(task_strs.len());

    for str in task_strs {
        let input = p_a_i(str);
        values.push(input);
    }

    let new_task = Task {
        id: id_task,
        name: values[0].to_string(),
        description: values[1].to_string(),
        date: values[2].to_string(),
        state: State::Completed,
    };
    new_task
}

fn add_task(json: &mut Tasks, new_task: Task) -> Result<(), Box<dyn Error>> {
    json.tasks.push(new_task);

    let new_file = File::create(DirFile::DIR).unwrap();
    serde_json::to_writer_pretty(new_file, &json)?;
    Ok(())
}

fn search_task<'a>(json: &'a Tasks, id: i32) -> Result<Vec<&'a Task>, String> {
    if id == 0 || id > json.tasks.len() as i32 {
        return Err("No se encontro la tarea".to_string());
    }
    Ok(json.tasks.iter().filter(|task| task.id == id).collect())
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

                        match add_task(&mut json, new_task) {
                            Ok(_) => println!("Tarea añadida exitosamente"),
                            Err(error) => println!("error: {}", error),
                        }
                    }
                    2 => {
                        let input = p_a_i("id");
                        let id: i32 = input.parse().unwrap_or(0);
                        match search_task(&json, id) {
                            Ok(task) => {
                                println!("{}", task[0].name);
                                println!("{}", task[0].description);
                            }
                            Err(error) => println!("{}", error),
                        }
                    }
                    3 => {
                        println!("Actualizar");
                        // ? Actualizar
                    }
                    4 => {
                        println!("Eliminar");
                        // ? Eliminar
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
