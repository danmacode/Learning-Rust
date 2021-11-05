extern crate dirs;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::{read, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct DataStruct {
    name: String,
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct JsonFile {
    number1: i32,
    field1: String,
    field2: Vec<DataStruct>,
}

fn read_json_file<P: AsRef<Path>>(
    path: P,
) -> std::result::Result<JsonFile, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let j = serde_json::from_reader(r)?;
    return Ok(j);
}

fn main() {
    let data = DataStruct {
        name: "name".to_string(),
        path: PathBuf::from(r"C:\Program Files\7-Zip"),
    };
    let json = JsonFile {
        number1: 2021,
        field1: "field1".to_string(),
        field2: vec![data],
    };
    let jstring: String = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", jstring);
    // https://www.reddit.com/r/rust/comments/38ka6i/how_to_close_a_file
    {
        // to close a filewriter automatically, open in a scope like this...
        let f = File::create("data.json").expect("Unable to create file");
        serde_json::to_writer_pretty(&f, &json);
        // or use drop(f) to take it out of scope explicitly
    }
    {
        // to read in one go as utf8 encoded text... and create Strings
        let f: Vec<u8> = std::fs::read("data2.json").expect("Unable to read file");
        let foo = String::from_utf8_lossy(&f);
        println!("Bytes vector lenght: {}", f.len());
        println!("{}", foo);
    }
    {
        // to read as a string
        let f = read("data2.json").expect("Unable to read file");
        let s = String::from_utf8_lossy(&f);
        let json2: JsonFile = serde_json::from_str(&s).unwrap();
    }
    // to read in a function that returns a Result,
    println!("===================================");
    let r = read_json_file("data3.json");
    let json2: JsonFile = if r.is_ok() { r.unwrap() } else { json };
    let jstring2: String = serde_json::to_string_pretty(&json2).unwrap();
    println!("{}", jstring2);
}
