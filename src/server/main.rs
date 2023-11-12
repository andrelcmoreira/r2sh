use std::env::args;
use std::io::{stdin, stdout, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::time::Duration;
use getopts::Options;

fn show_usage(progname: &str, opts: Options) {
    const BANNER: &str = "      ____      _\n  \
                           _ _|___ \\ ___| |__\n \
                          | '__|__) / __| '_ \\\n \
                          | |  / __/\\__ \\ | | |\n \
                          |_| |_____|___/_| |_|\n\
                         (r)ust(r)everse(sh)ell\n";
    let brief = format!("{BANNER}\nusage: {progname} [OPTIONS]");

    print!("{}", opts.usage(&brief));
}

fn parse_args() -> Option<u16> {
    let mut opts = Options::new();
    let args: Vec<String> = args().collect();

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

    let port: u16 = parsed_opts.opt_str("p").unwrap().parse().unwrap();

    Some(port)
}

fn read_cli_buffer(mut stream: &TcpStream) {
    let mut buf = [0; 1];

    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => print!("{}", buf[0] as char),
            Err(_) => break
        }
    };

    stdout().flush().unwrap();
}

fn handle_client(mut stream: &TcpStream) {
    let mut buffer: String = String::from("");

    // read the prompt
    read_cli_buffer(stream);

    loop {
        match stdin().read_line(&mut buffer) {
            Ok(_) => {},
            Err(_) => break
        };

        // issue the command
        stream.write(buffer.as_bytes()).unwrap();
        if buffer.eq("exit\n") {
            break;
        }

        // read the response
        read_cli_buffer(stream);
    }
}

fn run(port: u16) {
    println!("[+] starting server...");

    let sock = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(s) => s,
        Err(_) => panic!("[-] fail to bind the server to specified port!")
    };

    loop {
        let (cli_sock, cli_addr) = sock.accept().unwrap();

        println!("[+] client {} connected", cli_addr.ip());
        match cli_sock.set_read_timeout(Some(Duration::new(1, 0))) {
            Ok(_) => handle_client(&cli_sock),
            Err(_) => break
        };
    }
}

fn main() {
    let port = match parse_args() {
        Some(port) => port,
        None => exit(1)
    };

    run(port);
}
