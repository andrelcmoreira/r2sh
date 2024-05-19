use getopts::Options;

pub fn show_usage(progname: &String, opts: Options) {
    static BANNER: &str = "      ____      _\n  \
                           _ _|___ \\ ___| |__\n \
                          | '__|__) / __| '_ \\\n \
                          | |  / __/\\__ \\ | | |\n \
                          |_| |_____|___/_| |_|\n\
                         (r)everse(r)rust(sh)ell\n";
    let brief = format!("{BANNER}\nusage: {progname} [OPTIONS]");

    print!("{}", opts.usage(&brief))
}
