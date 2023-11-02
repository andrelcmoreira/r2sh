extern crate getopts;

use std::env::args;
use std::process::exit;

use getopts::Options;

struct R2shCtx {
    port: u16,
    addr: String
}

fn usage(progname: &str, opts: Options) {
    const BANNER: &str =
"     ____      _
  _ _|___ \\ ___| |__
 | '__|__) / __| '_ \\
 | |  / __/\\__ \\ | | |
 |_| |_____|___/_| |_|
(r)ust(r)everse(s)hell\n";
    let brief = format!("{BANNER}\nusage: {progname} [OPTIONS]");

    print!("{}", opts.usage(&brief));
}

fn parse_args() -> R2shCtx {
    let mut opts = Options::new();
    let args: Vec<String> = args().collect();

    opts.optopt("p", "port", "Specify the port of the server", "port");
    opts.optopt("s", "addr", "Specify the server address", "addr");
    opts.optflag("h", "help", "Show this message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("deu ruim") }
    };

    if ! matches.opt_present("s") || ! matches.opt_present("p")
        || matches.opt_present("h") {
        usage(args[0].as_str(), opts);
        // propagate error
    }

    let addr: String = matches.opt_str("s").unwrap();
    let port: u16 = matches.opt_str("p").unwrap().parse().unwrap();

    R2shCtx{ addr, port }
}

fn _exec_shell(fd: i8) -> () {

}

fn run(ctx: R2shCtx) -> i32 {
    0
}

fn main() {
    let ctx = parse_args();

    let ret = run(ctx);
    exit(ret);
}
