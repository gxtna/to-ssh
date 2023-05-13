use ssh2::Session;
use ssh_rs::{ssh, ShellBrocker};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
pub fn ssh_connect(comm: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut session = ssh::create_session()
        .username("ubuntu")
        .password("Gxt980926..")
        .connect("43.138.19.152:22")
        .unwrap()
        .run_backend();
    let mut shell = session.open_shell().unwrap();
    let x =run_shell(&mut shell,comm);
    shell.close().unwrap();
    //session.close();

    Ok(x)
}
fn run_shell(shell: &mut ShellBrocker, comm: String) -> String {
    let _vec = shell.read().unwrap();
    let mut str = comm;
    let enter = "\n".to_string();
    str += &enter;
    shell.write(str.as_bytes()).unwrap();
    sleep(Duration::from_secs(2));
    let vec = shell.read().unwrap();
    String::from_utf8(vec).unwrap()
}
