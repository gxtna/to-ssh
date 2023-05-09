use ssh2::Session;
use std::io::{stdin,stdout,Read,Write};
use std::net::TcpStream;
use std::thread;

pub fn ssh_connect()-> Result<(), Box<dyn std::error::Error>> {
    let mut sess = Session::new()?;
    sess.set_tcp_stream(TcpStream::connect("43.138.19.152:22")?);
    sess.handshake()?;
    sess.userauth_password("ubuntu", "Gxt980926..")?;
    let mut channel = sess.channel_session()?;
    channel.request_pty("xterm", None, None)?;
    channel.shell()?;
    channel.handle_extended_data(ssh2::ExtendedData::Merge)?;
    sess.set_blocking(false);
    let mut ssh_stdin = channel.stream(0);

    let stdin_thread = thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            let size = stdin().read(&mut buf).unwrap();
            ssh_stdin.write_all(&buf[..size]).unwrap();
        }
    });
    let stdout_thread = thread::spawn(move || {
        loop {
            let mut buf = [0; 1024];
            match channel.read(&mut buf) {
                Ok(c) if c > 0 => {
                    print!("{}", std::str::from_utf8(&buf).unwrap());
                    stdout().flush().unwrap();
                }
                Ok(0) => break,
                _ => (),
            }
        }
        channel.close().unwrap();
        print!("Exit: {}", channel.exit_status().unwrap());
    });

    stdin_thread.join().unwrap();
    stdout_thread.join().unwrap();
    Ok(())
}