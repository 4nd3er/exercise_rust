// * Desarrolla un programa que identifique archivos duplicados dentro de un directorio.
use std::{error::Error, fs::File, io::Read, path::Path};

const DIR: &str = "/Users/andero./downloads";

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
            let file = File::open(path)?;
            check_file(file);
        }
    }
    Ok(())
}

fn check_file(file: File) {
    let size = file.metadata().unwrap().len();
    println!("{} bytes", size);
    println!("{} mb", size / 1000 / 1000);
}

fn main() {
    match open_path() {
        Ok(path) => {
            iter_path(path).unwrap();
        }
        Err(error) => println!("error: {}.\n Try again later", error),
    }
}
