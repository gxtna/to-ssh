use ssh2::{Channel, Session};
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;
use std::thread;

pub fn ssh_connect() -> Result<String, Box<dyn std::error::Error>> {
    let mut sess = Session::new()?;
    sess.set_tcp_stream(TcpStream::connect("127.0.0.1:22")?);
    sess.handshake()?;
    sess.userauth_password("ubuntu", "123456")?;
    let mut channel = sess.channel_session()?;
    channel.request_pty("xterm", None, None)?;
    //channel.shell()?;
    //channel.handle_extended_data(ssh2::ExtendedData::Merge)?;
    //sess.set_blocking(false);
    channel.exec("ls").unwrap();
    channel.send_eof().unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().unwrap();
    channel.exit_status().unwrap();
    Ok(s)
}

