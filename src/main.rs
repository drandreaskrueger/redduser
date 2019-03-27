#![allow(dead_code)]
#![allow(unused_imports)]

use serde_json::{Result, Value};

use std::env;
use std::fs::File;
use std::io::Read;

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::{Utc};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

////////////////////////// deprecated  rustc_serialize
/*
extern crate rustc_serialize;
use rustc_serialize::json::Json;

fn read_json_file1(filename: String) -> Json {
    let path = env::current_dir().unwrap();
    println!(
        "The current directory is {:?}, reading file '{}':",
        path, filename
    );

    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    json
}
fn json_example1() {
    let json = read_json_file1(String::from("json-example.json"));
    println!("{}", json.find_path(&["Address", "Street"]).unwrap());
    println!(
        "{}",
        json.find_path(&["example_from", "stackoverflow"]).unwrap()
    );
}
*/
////////////////////////// not deprecated serde

/////////////// read in completely unstructured json

fn read_json_file_unstructured(filename: String) -> Value {
    let path = env::current_dir().unwrap();
    println!(
        "The current directory is {:?}, reading file '{}':",
        path, filename
    );
    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    json
}

fn json_example() {
    let json = read_json_file_unstructured(String::from("json-example.json"));
    println!("{}", json["Address"]["Street"]);
    println!("{}", json["example_from"]["serde.rs"]);
    println!("{:?}", json);
}

fn read_my_comments() {
    let json = read_json_file_unstructured(String::from("comments.json"));
    let json = json.as_array().unwrap().to_vec(); // we know that the json file is a list --> Vec
    println!("number of comments: {}", json.len());

    //println!("first element:\nutc: {}", json[0]["data"]["created_utc"]  );
    // println!("all elements:");

    for comment in json.iter() {
        let data = &comment["data"];

        let epoch : f64 = serde_json::from_value(data["created_utc"].clone()).unwrap(); // wow that is strangely complicated
        let ts = utc(epoch as u64);

        println!("{} name:{} parent:{}", ts, data["name"], data["parent_id"]  );
    }
}


fn utc(epochtime : u64) -> String{
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(epochtime as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


fn main() {
    // println!("Hello, world!");
    // json_example();

    read_my_comments()
}
