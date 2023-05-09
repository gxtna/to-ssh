


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod ssh_client;

fn main() {
    client::add_conn::get_client();
}


