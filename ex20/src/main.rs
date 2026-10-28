// * Busca archivos por nombre o extensión dentro de una carpeta y sus subcarpetas.
use std::{error::Error, fs, path::Path};

struct InfoFile {
    name: String,
    dir: String,
}

fn find_file(file: &str) -> Option<Vec<InfoFile>> {
    let path = Path::new("/Users/user/");
    let list_result = iter_dir(path, file).ok().unwrap_or(Vec::new());

    if list_result.len() == 0 {
        None
    } else {
        Some(list_result)
    }
}

fn iter_dir<'a>(path: &'a Path, file: &str) -> Result<Vec<InfoFile>, Box<dyn Error>> {
    let mut list_result: Vec<InfoFile> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
            .contains(file)
            || path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase()
                .contains(file)
        {
        // if path
        //     .file_stem()
        //     .map_or(false, |str| str.to_ascii_lowercase() == file)
        //     || path
        //         .extension()
        //         .map_or(false, |str| str.to_ascii_lowercase() == file)
        // {
            // ? otro metodo al condicional
            list_result.push(InfoFile {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                dir: path.to_string_lossy().to_string(),
            });
        }
        if path.is_dir() {
            // ! Asi solo no va a funcionar
            // iter_dir(&path, file)?;

            list_result.extend(iter_dir(&path, file).unwrap());
        }
    }
    Ok(list_result)
}

fn main() {
    let file = ".docx";
    match find_file(&file.to_lowercase()) {
        Some(files) => {
            for file in files {
                println!("Nombre del archivo: {}", file.name);
                println!("Dirección del archivo: {}", file.dir);
                println!("");
            }
        }
        None => println!("Archivo no encontrado"),
    }
}
