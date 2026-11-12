// * Desarrolla un programa que identifique archivos duplicados dentro de un directorio.
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    hash::Hash,
    io::Read,
    path::{Path, PathBuf},
};

const DIR: &str = "/Users/andero./downloads";

#[derive(Debug)]
struct HashFile {
    size: u64,
    frequenty: i32,
    dir: String,
}

fn open_path<'a>() -> Result<&'a Path, Box<dyn Error>> {
    let path = Path::new(DIR);
    Ok(path)
}

fn iter_path(path: &Path) -> Result<(), Box<dyn Error>> {
    for entry in path.read_dir().unwrap() {
        let entry = entry?;
        let path = entry.path();
        println!("{:?}", path);
        if !path.is_dir() {
            check_file(path);
        }
    }
    Ok(())
}

fn check_file(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut frequenty: HashMap<String, HashFile> = HashMap::new();
    let size = file.metadata().unwrap().len();
    println!("{} bytes", size);
    println!("{} mb", size / 1000 / 1000);
    match path.file_name() {
        Some(file_name) => {
            if !frequenty.contains_key(&file_name.to_string_lossy().to_string()) {
                frequenty.insert(
                    file_name.to_string_lossy().to_string(),
                    HashFile {
                        size,
                        frequenty: 1,
                        dir: path.to_string_lossy().to_string(),
                    },
                );
            } else {
                match frequenty.get_mut(&file_name.to_string_lossy().to_string()) {
                    Some(values) => {
                        values.frequenty += 1;
                    },
                    None => println!("hashMap no encontrado")
                }
            }
        }
        None => println!("No se encontró el nombre del archivo"),
    }
    println!("{:?}", frequenty);
    Ok(())
}

fn main() {
    match open_path() {
        Ok(path) => {
            iter_path(path).unwrap();
        }
        Err(error) => println!("error: {}.\n Try again later", error),
    }
}
