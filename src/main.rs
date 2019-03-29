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

// not completely unstructured - we know it's a list of json things, so make them a vec:

fn read_my_comments() {
    let json = read_json_file_unstructured(String::from("comments.json"));
    let json = json.as_array().unwrap().to_vec(); // we know that the json file is a list --> Vec
    println!("number of comments: {}", json.len());

    //println!("first element:\nutc: {}", json[0]["data"]["created_utc"]  );
    // println!("all elements:");

    for comment in json.iter() {
        println!();
        let data = &comment["data"]; // shortcut reference

        let created_utc = data["created_utc"].clone(); // wow this is strangely complicated
        let epoch : f64 = serde_json::from_value(created_utc).unwrap();
        // but these don't work (please tell me if you know a better way, me rust newbie):
        // let epoch : f64 = serde_json::from_value(data["created_utc"]).unwrap(); // cannot move out of borrowed content
        // let epoch : f64 = serde_json::from_value(&data["created_utc"]).unwrap(); // expected enum `serde_json::value::Value`, found reference
        let ts = utc(epoch as u64);

        println!("{} name:{} link:{} parent:{}", ts, data["name"], data["link_id"], data["parent_id"]  );

        let url = parent_url(&data);
    }
}

fn string_from_value(data : &Value) -> String {
    // is there an easier way than this ??
    // Looks like I have to do this for each and every element that I take out of the json??
    // why do I always have to clone the value 'data'??
    // IMHO there should be another fn like: serde_json::from_value_ref(&Value)
    let answer : String = serde_json::from_value(data.clone()).unwrap();
    answer
}

const REDDIT : &str = "https://www.reddit.com";
// sadly, format!(...) does not accept these consts, or what am I doing wrong?
const API_GET_T3_LINK    : &str = "/by_id/{}/.json"; // e.g. https://www.reddit.com/by_id/t3_b62s8t/.json
const API_GET_T1_COMMENT : &str = "/comments/{}/-/{}/.json"; // e.g. https://www.reddit.com/comments/b5ymaf/-/ejhbxes/.json


fn parent_url(data : &Value) -> String{

    let parent_id = string_from_value(& data["parent_id"]);
    let link_id = string_from_value(& data["link_id"]);

    let mut url = String::from(REDDIT);

    if &parent_id[0..3] == "t1_" {
        print!("COMMENT parent {} --> ", &parent_id[3..]);
        url += &format!("/comments/{}/-/{}/.json", &link_id[3..], &parent_id[3..]); // sad, format!() does not seem to take constants
    };
    if &parent_id[0..3] == "t3_" {
        print!("LINK parent {} --> ", parent_id);
        url += &format!("/by_id/{}/.json", &parent_id);
    }
    println!("{}", url);

    url
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
