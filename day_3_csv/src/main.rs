use csv::Writer;
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct NaiveCSVWriter {
    contents: Vec<Vec<String>>,
}

impl NaiveCSVWriter {
    fn from_file(path: &str) -> Option<NaiveCSVWriter> {
        let file = File::open(path).ok()?;
        let bf = BufReader::new(file);
        let mut contents: Vec<Vec<String>> = vec![];

        for line in bf.lines() {
            match line {
                Ok(line) => contents.push(
                    line.split(",")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ),
                Err(_) => continue,
            }
        }

        Some(NaiveCSVWriter { contents: contents })
    }
}

#[derive(Serialize, Debug)]
struct FilmRow {
    name: String,
    director: String,
    year: usize,
}

impl FilmRow {
    fn new(name: &str, director: &str, year: usize) -> Self {
        FilmRow {
            name: String::from(name),
            director: String::from(director),
            year: year,
        }
    }
}

fn write_csv(path: &str) {
    let dollar_films = vec![
        FilmRow::new("A Fistful of Dollars", "Rojo", 1964),
        FilmRow::new("For a Few Dollars More", "El Indio", 1965),
        FilmRow::new("The Good the Bad and the Ugly", "Tuco", 1966),
    ];
    let mut writer = Writer::from_path(path).unwrap();
    for film in dollar_films {
        writer.serialize(film).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Writing 3 rows in csv file");
    write_csv(&filename);
    println!("Reading file {}", filename);
    let writer = NaiveCSVWriter::from_file(&filename).unwrap();
    for row in writer.contents {
        println!("{:?}", row);
    }
}
