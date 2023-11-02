extern crate getopts;

use std::env::args;
use std::process::exit;
use getopts::Options;

struct R2shCtx {
    port: u16,
    addr: String
}

fn usage(progname: &str, opts: Options) {
    // TODO(andrelcmoreira) improve this identation
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

fn parse_args() -> Option<R2shCtx> {
    let mut opts = Options::new();
    let args: Vec<String> = args().collect();

    opts.optopt("p", "port", "Specify the port of the server", "port");
    opts.optopt("s", "addr", "Specify the server address", "addr");
    opts.optflag("h", "help", "Show this message");

    if args.len() == 1 {
        usage(args[0].as_str(), opts);
        return None
    }

    let parsed_opts = opts.parse(&args[1..]).unwrap();
    if ! parsed_opts.opt_present("s") || ! parsed_opts.opt_present("p")
        || parsed_opts.opt_present("h") {
        usage(args[0].as_str(), opts);
        return None
    }

    let addr: String = parsed_opts.opt_str("s").unwrap();
    let port: u16 = parsed_opts.opt_str("p").unwrap().parse().unwrap();

    Some(R2shCtx{ addr, port })
}

fn _exec_shell(_fd: i8) {

}

fn run(_ctx: R2shCtx) -> i32 {
    0
}

fn main() {
    let ctx = match parse_args() {
        Some(ctx) => ctx,
        None => exit(1)
    };

    let ret = run(ctx);
    exit(ret);
}
