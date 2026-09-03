use std::io;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = mochiko_cli::cli::dispatch(&args, &mut io::stdout(), &mut io::stderr());
    process::exit(code);
}
