use std::env::args;
use std::io::{stdin, stdout, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::time::Duration;
use getopts::Options;

mod common;
use common::{set_panic_handler, show_usage};

fn parse_args() -> Option<u16> {
    let mut opts = Options::new();
    let args = args().collect::<Vec<String>>();

    opts.optopt("p", "port", "Specify the port to bind the server to", "port");
    opts.optflag("h", "help", "Show this message");

    if args.len() == 1 {
        show_usage(args[0].as_str(), opts);
        return None
    }

    let parsed_opts = opts.parse(&args[1..]).unwrap();
    if ! parsed_opts.opt_present("p") || parsed_opts.opt_present("h") {
        show_usage(args[0].as_str(), opts);
        return None
    }

    let port = parsed_opts.opt_str("p")?;

    Some(port.parse().unwrap())
}

fn read_cli_buffer(mut stream: &TcpStream) {
    let mut buf = [0; 1];

    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => {
                print!("{}", buf[0] as char);
                stdout().flush().unwrap()
            },
            Err(_) => break
        }
    }
}

fn handle_client(mut stream: &TcpStream) {
    let mut buffer = String::new();

    loop {
        read_cli_buffer(stream);

        // read user command
        stdin().read_line(&mut buffer).unwrap();
        // issue the command
        stream.write(buffer.as_bytes()).unwrap();
        if buffer.eq("exit\n") {
            break
        }

        buffer.clear()
    }
}

fn run(port: u16) {
    println!("[+] starting server...");

    let sock = TcpListener::bind(("127.0.0.1", port))
        .expect("[-] fail to bind the server to specified port!");

    loop {
        let (cli_sock, cli_addr) = sock.accept().unwrap();

        cli_sock.set_read_timeout(Some(Duration::new(1, 0)))
            .expect("[-] fail to set the timeout for socket operations!");

        println!("[+] client {} connected", cli_addr.ip());
        handle_client(&cli_sock);
        println!("[+] client {} disconnected", cli_addr.ip())
    }
}

fn main() {
    set_panic_handler();

    match parse_args() {
        Some(port) => run(port),
        None => exit(1)
    }
}
