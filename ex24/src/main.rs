// * Desarrolla un programa que identifique archivos duplicados dentro de un directorio.
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Read,
    path::{Path, PathBuf},
};

const DIR: &str = "/Users/andero./downloads";

#[derive(Debug)]
struct HashFile {
    name: String,
    size: u64,
    frequenty: i32,
    dir: Vec<String>,
}

fn separator () {
    println!("{:#^30}", "");
}

fn open_path<'a>() -> Result<&'a Path, Box<dyn Error>> {
    let path = Path::new(DIR);
    Ok(path)
}

fn iter_path(path: &Path) -> Result<HashMap<String, HashFile>, Box<dyn Error>> {
    let mut frequenty: HashMap<String, HashFile> = HashMap::new();
    for entry in path.read_dir().unwrap() {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let _ = check_file(path, &mut frequenty);
        }
    }
    Ok(frequenty)
}

fn check_file(
    path: PathBuf,
    frequenty: &mut HashMap<String, HashFile>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&path)?;
    let size = file.metadata().unwrap().len();

    match path.file_name() {
        Some(file_name) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if content.len() == 0 {
                return Ok(());
            }

            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);


            if !frequenty.contains_key(&hasher.finish().to_string()) {
                frequenty.insert(
                    hasher.finish().to_string(),
                    HashFile {
                        name: file_name.to_string_lossy().to_string(),
                        size,
                        frequenty: 1,
                        dir: vec![path.to_string_lossy().to_string()],
                    },
                );
            } else {
                match frequenty.get_mut(&hasher.finish().to_string()) {
                    Some(values) => {
                        values.frequenty += 1;
                        values.dir.push(path.to_string_lossy().to_string());
                    }
                    None => println!("hashMap no encontrado"),
                }
            }
        }
        None => println!("No se encontró el nombre del archivo"),
    }
    Ok(())
}

fn main() {
    match open_path() {
        Ok(path) => {
            let duplicates = iter_path(path).unwrap();
            for (_, value) in duplicates {
                if value.frequenty >= 2 {
                    separator();
                    println!("Elementos duplicados");
                    value.dir.iter().for_each(|d| println!("{}", d));
                    separator();
                    println!("");
                }
            }
        }
        Err(error) => println!("error: {}.\n Try again later", error),
    }
}
