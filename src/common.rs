use getopts::Options;

pub fn show_usage(progname: &str, opts: Options) {
    static BANNER: &str = "      ____      _\n  \
                           _ _|___ \\ ___| |__\n \
                          | '__|__) / __| '_ \\\n \
                          | |  / __/\\__ \\ | | |\n \
                          |_| |_____|___/_| |_|\n\
                         (r)ust(r)everse(sh)ell\n";
    let brief = format!("{BANNER}\nusage: {progname} [OPTIONS]");

    print!("{}", opts.usage(&brief));
}
