extern crate getopts;

use std::env::args;
use std::process::exit;
use getopts::Options;

struct R2shCtx {
    port: u16,
    addr: String
}

fn show_usage(progname: &str, opts: Options) {
    const BANNER: &str = "      ____      _\n  \
                           _ _|___ \\ ___| |__\n \
                          | '__|__) / __| '_ \\\n \
                          | |  / __/\\__ \\ | | |\n \
                          |_| |_____|___/_| |_|\n\
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
        show_usage(args[0].as_str(), opts);
        return None
    }

    let parsed_opts = opts.parse(&args[1..]).unwrap();
    if ! parsed_opts.opt_present("s") || ! parsed_opts.opt_present("p")
        || parsed_opts.opt_present("h") {
        show_usage(args[0].as_str(), opts);
        return None
    }

    let addr: String = parsed_opts.opt_str("s").unwrap();
    let port: u16 = parsed_opts.opt_str("p").unwrap().parse().unwrap();

    Some(R2shCtx{ addr, port })
}

fn exec_shell(_fd: i8) {

}

fn run(_ctx: R2shCtx) -> i32 {
    exec_shell(0);

    return 0;  // normally never reached
}

fn main() {
    let ctx = match parse_args() {
        Some(ctx) => ctx,
        None => exit(1)
    };

    let ret = run(ctx);
    exit(ret);
}
