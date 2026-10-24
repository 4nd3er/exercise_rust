// * Lee un CSV con registros numéricos, calcula estadísticas básicas y genera un archivo de salida.
use std::{error::Error};
use std::fs::{File, OpenOptions};
use std::path::Path;
use csv::{Reader, Writer, WriterBuilder};

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(&filename)?;
    let mut rdr = Reader::from_reader(file);

    let mut total_price = 0;
    let mut total_quantity = 0;
    for result in rdr.records() {
        let record = result?;
        for string in &record {
            println!("{:?}", string);
            let split: Vec<&str> = string.split(";").collect();
            total_price += split[split.len() - 2].trim().parse().unwrap_or(0);
            total_quantity += split[split.len() - 1].trim().parse().unwrap_or(0);
            println!(" ");
        }
    }

    // ? Reescribir el archivo
    // let file_changed = OpenOptions::new().append(true).open(&filename)?;
    // let mut writer = Writer::from_writer(file_changed);
    // ? Reescribir el archivo

    let mut writer = WriterBuilder::new()
    .has_headers(true)
    .from_path("test.csv")?;
    writer.write_record(["Campo", "Total"])?;
    writer.write_record(["precio", total_price.to_string().as_str()])?;
    writer.write_record(["cantidad", total_quantity.to_string().as_str()])?;

    Ok(())
}

fn main() {
    let filename = "Copia de productos.csv";
    if let Err(err) = read_csv(filename) {
        println!("error running example: {}", err);
    }
}
