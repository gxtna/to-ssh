use crate::ssh_client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnInfo {
    host: String,
    port: i32,
    username: String,
    password: String,
}

#[tauri::command]
pub fn add_connection(conn: ConnInfo) -> bool {
    let json = serde_json::to_string(&conn).unwrap();
    let path = "./config.json".to_string();
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    file.write_all(String::as_bytes(&String::from(json + "\n")))
        .unwrap();

    true
}

#[tauri::command]
fn ssh_client(comm: String) -> String {
    let x = ssh_client::ssh::ssh_connect(comm).unwrap();
    println!("{}",x);
    println!("{}","asdasdasd");
    x
}

#[tauri::command]
fn read_config() -> Vec<ConnInfo> {
    let mut res = Vec::new();
    let path = "./config.json".to_string();
    let file = File::open(path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let data: ConnInfo = serde_json::from_str(&line.unwrap().to_string()).unwrap();
                res.push(data);
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    res
}

pub fn get_client() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_config,
            add_connection,
            ssh_client
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
