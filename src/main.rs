#![allow(dead_code)]
#![allow(unused_imports)]

use serde_json::{Result, Value};

use std::env;
use std::fs::File;
use std::io::Read;

extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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


fn utc(epochtime: u64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(epochtime as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn get_data_from_comment_json_and_print_statusline (comment: &Value)  -> &Value {
    println!();
    let data = &comment["data"]; // shortcut reference

    let created_utc = data["created_utc"].clone(); // wow this is strangely complicated
    let epoch: f64 = serde_json::from_value(created_utc).unwrap();
    // but these don't work (please tell me if you know a better way, me rust newbie):
    // let epoch : f64 = serde_json::from_value(data["created_utc"]).unwrap(); // cannot move out of borrowed content
    // let epoch : f64 = serde_json::from_value(&data["created_utc"]).unwrap(); // expected enum `serde_json::value::Value`, found reference
    let ts = utc(epoch as u64);

    println!(
        "{} name:{} link:{} parent:{}",
        ts, data["name"], data["link_id"], data["parent_id"]
    );

    data
}

fn string_from_value(data: &Value) -> String {
    // is there an easier way than this ??
    // Looks like I have to do this for each and every element that I take out of the json??
    // why do I always have to clone the value 'data'??
    // IMHO there should be another fn like: serde_json::from_value_ref(&Value)
    let answer: String = serde_json::from_value(data.clone()).unwrap();
    answer
}

const REDDIT: &str = "https://www.reddit.com";
// sadly, format!(...) does not accept these consts, or what am I doing wrong?
const API_GET_T3_LINK: &str = "/by_id/{}/.json"; // e.g. https://www.reddit.com/by_id/t3_b62s8t/.json
const API_GET_T1_COMMENT: &str = "/comments/{}/-/{}/.json"; // e.g. https://www.reddit.com/comments/b5ymaf/-/ejhbxes/.json

fn ancestor_urls(data: &Value) -> Vec<String> {
    // not String but Vec because answer can be 1 or 2 URLs

    let parent_id = string_from_value(&data["parent_id"]);
    let link_id = string_from_value(&data["link_id"]);

    let mut urls: Vec<String> = [].to_vec();

    match &parent_id[0..3] {
        "t1_" => {
            print!("COMMENT parent {} --> ", &parent_id[3..]);
            urls.push(String::from(REDDIT) + &format!("/comments/{}/-/{}/.json", &link_id[3..], &parent_id[3..]));
            urls.push(String::from(REDDIT) + &format!("/by_id/{}/.json", &link_id)); // also OP of this thread
        }
        "t3_" => {
            print!("LINK parent {} --> ", parent_id);
            urls.push(String::from(REDDIT) + &format!("/by_id/{}/.json", &parent_id));
        }
        _ => print!("WARN: id type not implemented yet: '{}'. Skipping this. --> ", parent_id)
    }
    println!("{:?}", urls);

    urls
}

fn sort_and_deduplicate(mut strvec : Vec<String>) {
    // deduplicate:
    print!("number of strings: {} ", strvec.len());
    print!("... dedup ... ");
    strvec.sort_unstable();
    strvec.dedup();
    println!("number of strings: {}", strvec.len());

}


fn read_my_comments_and_create_ancestor_urls() {
    let json = read_json_file_unstructured(String::from("comments.json"));
    let json_vec = json.as_array().unwrap().to_vec(); // we know the json file is a list --> Vec
    println!("number of comments: {}", json_vec.len());

    let mut urls: Vec<String> = [].to_vec();
    for comment in json_vec.iter() {
        let data = get_data_from_comment_json_and_print_statusline(&comment);
        urls.extend( ancestor_urls(&data) );
    }
    println!("\n{:?}", urls);
    sort_and_deduplicate(urls);

}



fn main() {
    // println!("Hello, world!");
    // json_example();

    read_my_comments_and_create_ancestor_urls();


}
