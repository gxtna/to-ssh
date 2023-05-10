use ssh2::{Channel, Session};
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;
use std::thread;

pub fn ssh_connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut sess = Session::new()?;
    sess.set_tcp_stream(TcpStream::connect("127.0.0.1:22")?);
    sess.handshake()?;
    sess.userauth_password("ubuntu", "123456")?;
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
    /* let stdout_thread = thread::spawn(move || {
        loop {
            let mut buf = [0; 1024];
            match channel.read(&mut buf) {
                Ok(c) if c > 0 => {
                    print!("{}", std::str::from_utf8(&buf).unwrap());
                     stdout().flush().unwrap();
                }
                Ok(0) => break,
                _ => (),
            };
        }

        channel.close().unwrap();
        print!("Exit: {}", channel.exit_status().unwrap());
    }); */
    //let res = thread::spawn(move || {
        let mut datas = Vec::new();
        loop {
            let mut buf = [0; 1024];
        
            let data = match channel.read(&mut buf) {
                Ok(c) if c > 0 => {
                    datas.push(std::str::from_utf8(&buf).unwrap().to_string());
                    println!("{}",std::str::from_utf8(&buf).unwrap().to_string());
                    stdout().flush().unwrap();
                }
                Ok(0) => break,
                _ => (),
            };
        }
        println!("{}",datas.len());
        channel.close().unwrap();
        //datas

    //});
    //let x = res.join().unwrap();
    //println!("{:?}",x.len());
    /* for ele in x {
        if ele.len()>0 {
            println!("{:?}",ele);
        }
    } */
    stdin_thread.join().unwrap();
    //res.join().unwrap();
    //stdout_thread.join().unwrap();
    Ok(())
}

fn read_data(mut channel: Channel) {
    let res = thread::spawn(move || {
        let mut datas = Vec::new();
        loop {
            let mut buf = [0; 1024];
            let data = match channel.read(&mut buf) {
                Ok(c) if c > 0 => {
                    std::str::from_utf8(&buf).unwrap().to_string()
                }
                Ok(0) => break,
                _ => "".to_owned(),
            };
            datas.push(data);
        }
        datas
    });
    let x = res.join().unwrap();
    println!("{:?}",x.len());
    for ele in x {
        println!("{:?}",ele);
    }

}
