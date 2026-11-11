// * Desarrolla un programa que lea archivos .log y extraiga información relevante.

use std::{collections::HashMap, error::Error, fs::File, io::Read};

const DIR: &str = "rs.log";

fn open_log() -> Result<File, Box<dyn Error>> {
    let file = File::open(DIR)?;
    Ok(file)
}

fn main() {
    match open_log() {
        Ok(mut file) => {
            let mut logs: HashMap<&str, Vec<String>> = HashMap::new();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let registers: Vec<&str> = input.split("\n").collect();
            println!("Total registros: {}", registers.len());
            for register in registers {
                let type_log_v: Vec<&str> = register.split(" ").collect();
                let info_v: Vec<&str> = register.split("-").collect();
                let type_log = type_log_v[2].trim();
                if !logs.contains_key(type_log) {
                    logs.insert(type_log, vec![]);
                }
                match logs.get_mut(type_log) {
                    Some(list_logs) => {
                        list_logs.push(info_v[info_v.len() - 1].trim().to_string());
                    }
                    None => println!("Tipo de error no encontrado"),
                }
            }
            let mut frequenty_info: HashMap<String, i32> = HashMap::new();
            for (type_log, list_log) in logs {
                println!("{type_log} {:?}", list_log);
                println!("");
                for log in list_log {
                    if !frequenty_info.contains_key(log.as_str()) {
                        frequenty_info.insert(log, 1);
                    } else {
                        match frequenty_info.get_mut(&log) {
                            Some(num) => {
                                *num += 1;
                            },
                            None => println!("No se encontró el valor")
                        }
                    }
                }
            }

            let mut max = 0;
            let mut max_info_currency = String::new();
            for (key, value) in frequenty_info {
                if value > max {
                    max = value;
                    max_info_currency = key;
                }
            }
            println!("Info mas frecuente: {} con {} ocurrencias", max_info_currency, max);
        }
        Err(error) => println!("error: {}", error),
    }
}
