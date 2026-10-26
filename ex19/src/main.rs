// * Consulta una API pública, procesa la respuesta JSON y muestra unos campos seleccionados.
use reqwest;
use std::error::Error;
use json;

fn api () -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://rickandmortyapi.com/api/character")?.text()?;

    let json_value = json::parse(&resp)?;

    for character in json_value["results"].members() {
        println!("{:#^20}", "");
        println!("Nombre: {}", character["name"]);
        println!("Estado: {}", character["status"]);
        println!("Especie: {}", character["species"]);
        println!("Genero: {}", character["gender"]);
        println!("{:#^20}", "");
        println!("");
    }

    Ok(())
}

fn main() {
    if let Err(err) = api() {
        println!("error: {}", err);
    }
}
