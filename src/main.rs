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

fn ancestor_url(data: &Value) -> (String,String){

    let parent_id = string_from_value(&data["parent_id"]);
    let link_id = string_from_value(&data["link_id"]);

    let mut url = String::from("");
    let mut name = String::from("");

    match &parent_id[0..3] {
        "t1_" => {
            print!("parent COMMENT {} and ancestor THREAD-OP --> ", &parent_id[3..]);
            url = String::from(REDDIT) + &format!("/comments/{}/-/{}/.json", &link_id[3..], &parent_id[3..]);
            name = parent_id;
            // thread_op = String::from(REDDIT) + &format!("/by_id/{}/.json", &link_id); // separately OP of this thread; not needed, included anyways
        }
        "t3_" => {
            print!("parent THREAD-OP {} --> ", parent_id);
            url = String::from(REDDIT) + &format!("/by_id/{}/.json", &parent_id);
            name = parent_id;
        }
        _ => print!("WARN: id type not implemented yet: '{}'. Skipping this. --> ", parent_id)
    }
    println!("{} {}", name, url);

    (name, url)
}

fn sort_and_deduplicate_and_remove_empty(strvec : &mut Vec<(String,String)>) {
    // deduplicate:
    print!("number of strings: {} ", strvec.len());
    print!("... dedup ... ");
    strvec.sort_unstable(); // sort, needed for dedup
    strvec.dedup(); // remove all duplicates
    //strvec.retain(|x| x != ""); // keep all non-empty
    println!("number of strings: {}", strvec.len());
}

fn download_json(url: &String) -> Value{
    // let body = reqwest::get("https://www.rust-lang.org").unwrap().text();
    let mut answer = reqwest::get(url).unwrap();
    // let body  = answer.text().unwrap();
    // let json : Value = serde_json::from_str(&body).unwrap();
    let json : Value = answer.json().unwrap();
    // println!("body = {:?}", json);
    json
}

fn download_json_comments(url: &String) -> Value{
    let json = download_json(&url);
    let json_vec = json.as_array().unwrap().to_vec(); // we know the json file is a list --> Vec
    println!("number of elements: {}", json_vec.len());


    // TODO: work through threadOP, and always return this too, see below.
    // println!("0 == threadOP");
    //println!("{:?}", json_vec[0]);


    println!("1 == comments");

    let comments_children = &json_vec[1]["data"]["children"];  // .[1].data.children[0].data
    let num_children = comments_children.as_array().unwrap().to_vec().len();

    match num_children {
        0 => {
            println!("INFO: comment DELETED");
            return serde_json::from_str("").unwrap();
        },
        1 => {
            let comments = &comments_children[0]["data"];
            print!("INFO: comment {} ", comments["name"]);
            if comments["replies"] == "" {
                println!("has no further replies ");
            }
            else{
                let num_replies = comments["replies"]["data"]["children"].as_array().unwrap().to_vec().len();
                println!("has {} replies below. Could later be used to reduce number of downloads. Ignoring for now.", num_replies);
            }
            return comments.clone();  // not sure this is the best way
        },
        _ => {
            println!("WARN: answer has {} > 1 children, attention", num_children);
            return serde_json::from_str("").unwrap();
        }
    }


}


fn download_json_threadops(url: &String) {  //-> Value{
    let json = download_json(&url);
    let obj = &json["data"];
    println!("{:?}", obj);
}

fn read_my_comments_and_create_ancestor_urls() -> Vec<(String,String)>{
    let json = read_json_file_unstructured(String::from("comments.json"));
    let json_vec = json.as_array().unwrap().to_vec(); // a list of json things, so make them a vec
    println!("number of comments: {}", json_vec.len());

    let mut urls = Vec::<(String,String)>::new();

    for comment in json_vec.iter() {
        let data = get_data_from_comment_json_and_print_statusline(&comment);
        let (name, url) = ancestor_url(&data);
        urls.push( (name, url) );
    }
    println!("\nurls: {:?}", urls);
    sort_and_deduplicate_and_remove_empty(&mut urls);
    // println!("\nurls: {:?}", urls);

    urls
}



fn main() {
    // println!("Hello, world!");
    // json_example();

    // read_my_comments_and_create_ancestor_urls();

    download_json_comments(&String::from("https://www.reddit.com/comments/b4znbi/-/ejg8fwk/.json")); // comment with OP that has a title AND a selftext
    download_json_comments(&String::from("https://www.reddit.com/r/politics/comments/bdwg0g/-/el29x7x/.json")); // comment with subcomments
    // download_json_threadops(&String::from("https://www.reddit.com/by_id/t3_ba2a3a/.json"));
}
