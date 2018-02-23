#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;
extern crate url;

use url::Url;

use rocket::State;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

//sqlite DB setup
extern crate rusqlite;

use rusqlite::Connection;
use rusqlite::Result;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::{Json, Value};

//Configuration
extern crate ini;
use ini::Ini;

#[derive(Deserialize, Serialize, Clone)]
struct ClientMessage {
    user_name: String,
    message: String,
    time: String
}

struct MetaData {
    client_data: Arc<Mutex<ClientData>>,
    sqlite_db: Arc<Mutex<Connection>>,
    server_ip: String
}

struct ClientData {
    session_id: String,
    user_name: String,
    local_ip: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<user>")]
fn hello_user(user: String) -> String {
    format!("Hello, {}!", user)
}

#[get("/register/<name>")]
fn register_me(name: String, meta_data: State<MetaData>) -> String {
    let mut mut_client_data = meta_data.client_data.lock().unwrap();
    let uri_string = format!("http://{}:8000/register/{}/{}", meta_data.server_ip,
                             name, mut_client_data.local_ip);
    //    println!("uri_string {}",uri_string);
    let uri: Url = uri_string.parse().unwrap();
    let mut response = reqwest::get(uri).unwrap();
    let session_id = response.text().unwrap();
    if session_id == "0".to_string() {
        println!("response {}", session_id);
        "This name is already registered, please log out and try again".to_string()
    } else {
        mut_client_data.session_id = session_id;
        mut_client_data.user_name = name;
        println!("response code {}", response.status());
        "registered u".to_string()
    }
}

#[get("/send/<message>")]
fn send_msg(message: String, meta_data: State<MetaData>) {
    let client_data = meta_data.client_data.lock().unwrap();
    let uri_string = format!("http://{}:8000/broadcast",meta_data.server_ip);
    let uri: Url = uri_string.parse().unwrap();

    let mut map = HashMap::new();
    map.insert("source_ip", &client_data.local_ip);
    map.insert("user_name", &client_data.user_name);
    map.insert("session_id", &client_data.session_id);
    map.insert("message", &message);

    let client = reqwest::Client::new();
    let res = client.post(uri)
        .json(&map)
        .send().unwrap();
}

#[get("/receive/<user_name>/<message>/<time>")]
fn receive_msg(user_name: String, time: String, message: String, meta_data: State<MetaData>) {
    let mut_sqlite_conn = meta_data.sqlite_db.lock().unwrap();
    println!("received message {} and storing it locally on client", message);
    mut_sqlite_conn.execute("INSERT INTO messages(user_name, message, time) VALUES(?1,?2,?3)",
                            &[&user_name, &message, &time]).unwrap();
    ()
}

#[get("/get/messages/<count>")]
fn get_messages(count: i64, meta_data: State<MetaData>) -> String {
    let sqlite_conn = meta_data.sqlite_db.lock().unwrap();
    let mut stmt = sqlite_conn
        .prepare("SELECT id, user_name, message, time FROM messages order by id desc limit ?1").unwrap();
    let client_messages = stmt.query_map(&[&count], |row| {
        ClientMessage {
            user_name: row.get(1),
            message: row.get(2),
            time: row.get(3)
        }
    }).unwrap();

    let messages: Vec<String> = client_messages.map(|row| {
        println!("read message");
        let unwrapped_row = row.unwrap();
        let clone_row = unwrapped_row.clone();
        let mut message_metadata = String::new();
        let user_name: String = clone_row.user_name;
        let message: String = clone_row.message;
        let time: String = clone_row.time;
        message_metadata.push_str("{ \"user_name\":");
        message_metadata.push_str(&user_name.to_owned());
        message_metadata.push_str(", \"message\":");
        message_metadata.push_str(&message.to_owned());
        message_metadata.push_str(", \"time\":");
        message_metadata.push_str(&time.to_owned());
        message_metadata.push_str("}");
        message_metadata
    }).collect();
    messages.join(",")
}

#[get("/logout")]
fn logout(meta_data: State<MetaData>) -> String {
    let mut mut_client_data = meta_data.client_data.lock().unwrap();
    let uri_string = format!("http://{}:8000/logout/{}/{}/{}",meta_data.server_ip,
                             mut_client_data.session_id,mut_client_data.user_name,
                             mut_client_data.local_ip);
    let uri: Url = uri_string.parse().unwrap();
    let mut response = reqwest::get(uri).unwrap();
    //clearing client data on logout response
    mut_client_data.session_id = "".to_string();
    mut_client_data.user_name = "".to_string();

    response.text().unwrap()
}

fn init() -> MetaData {
    let conn = Connection::open("messages.db").unwrap();
    //Create table to store messages
    conn.execute("CREATE TABLE IF NOT EXISTS messages( \
    id INTEGER PRIMARY KEY,\
    user_name TEXT NOT NULL,\
    message TEXT NOT NULL,\
    time TEXT NOT NULL)", &[]).unwrap();

    let conf = Ini::load_from_file("conf.ini").unwrap();

    let section = conf.section(Some("Config".to_owned())).unwrap();
    let local_ip = section.get("local_ip").unwrap();
    let server_ip = section.get("server_ip").unwrap();

    //State
    MetaData {
        client_data: Arc::new(Mutex::new(ClientData { session_id: "".to_string(), user_name: "".to_string(), local_ip: local_ip.to_string() })),
        sqlite_db: Arc::new(Mutex::new(conn)),
        server_ip: server_ip.to_string()
    }
}

fn main() {
    rocket::ignite()
        .manage(init())
        .mount("/", routes![index,hello_user,register_me,send_msg,receive_msg,get_messages,logout])
        .launch();
}