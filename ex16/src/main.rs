// * Permite crear, buscar, editar y eliminar contactos guardados en memoria.
use std::io::stdin;

#[derive(Debug)]
struct User {
    name: String,
    age: String,
    email: String,
}

fn p(string: &str) {
    println!("{string}");
}

fn get_new_user(string_users: &Vec<&str>) -> User {
    let mut vector = vec![String::new(), String::new(), String::new()];

    for i in 0..string_users.len() {
        p(string_users[i]);
        let _ = stdin().read_line(&mut vector[i]);
        vector[i] = vector[i].trim().to_string();
    }
    let user = User {
        name: vector[0].clone(),
        age: vector[1].clone(),
        email: vector[2].clone(),
    };
    user
}

fn in_a_pr(string: &str) -> String {
    p(string);
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input
}

fn add_user(users: &mut Vec<User>, user: User) {
    users.push(user);
}

fn find_user<'a>(users: &'a mut Vec<User>, email: &str) -> Option<(usize, &'a mut User)> {
    // users
    //     .iter()
    //     .filter(|user| user.email == email)
    //     .collect::<Vec<_>>()
    // * Funciona

    // ? codigo IA (mejor claramente)
    if let Some((position_user, user)) = users
        .iter_mut()
        .enumerate()
        .find(|(_, user)| user.email == email)
    {
        return Some((position_user, user));
    } else {
        return None;
    }
    // ? codigo IA (mejor claramente)
}

fn show_users(users: &Vec<User>) {
    for user in users {
        p(&user.name);
        p(&user.age);
        p(&user.email);
    }
}

fn edit_user(user: &mut User) {
    p("Nombre (1)");
    p("Edad (2)");
    p("email (3)");
    let field = in_a_pr("Digite el campo que desea editar");

    let field: i32 = field.trim().parse().expect("Numero no valido");
    let new_value = in_a_pr("Digita el nuevo valor").trim().to_string();
    match field {
        1 => user.name = new_value,
        2 => user.age = new_value,
        3 => user.email = new_value,
        _ => p("tal cosa"),
    }
}

fn remove_user(users: &mut Vec<User>, position: usize) {
    users.remove(position);
}

fn main() {
    let mut users = Vec::<User>::new();
    let string_users = vec!["Digita el nombre", "Digita la edad", "Digita el correo"];

    loop {
        let mut input = String::new();

        p("Que quieres hacer?");
        p("Añadir usuario (1)");
        p("Buscar usuario (2)");
        p("Editar usuario (3)");
        p("Eliminar usuario (4)");
        let _ = stdin().read_line(&mut input);

        if input.trim() == "q" {
            break;
        }

        let instruction: i32 = input.trim().parse().expect("Digito no valido");

        match instruction {
            1 => {
                let user = get_new_user(&string_users);
                add_user(&mut users, user);
                p("Usuario añadido exitosamente!");
                p("");
            }
            2 => {
                let result = in_a_pr("Digita el correo");
                let result = find_user(&mut users, result.trim());

                if let Some((_, user)) = result {
                    p(&user.name);
                } else {
                    p("Usuario no encontrado");
                }
            }
            3 => {
                let email = in_a_pr("Digita el correo");

                let result = find_user(&mut users, email.trim());
                if let Some((_, user)) = result {
                    edit_user(user);
                    p("Usuario editado satisfactoriamente!");
                } else {
                    p("Usuario no encontrado");
                }
            }
            4 => {
                let email = in_a_pr("Digita el correo");

                let result = find_user(&mut users, email.trim());
                if let Some((position, _)) = result {
                    remove_user(&mut users, position);
                    p("Usuario eliminado satisfactoriamente!");
                } else {
                    p("Usuario no encontrado");
                }
            },
            5 => show_users(&users),
            _ => p("Instrucción no valida"),
        }
    }
}
