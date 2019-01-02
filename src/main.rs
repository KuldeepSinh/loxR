use std::env::{self, Args};
use std::process;

fn main() {
    let mut args = env::args();
    args.next();
    if args.len() > 2 {
        println!("Too many arguments. Usage: loxr [script]",);
        process::exit(64);
    } else if args.len() == 2 {
        args.next();
        run_file(args.next());
    } else {
        run_prompt();
    }
}

fn run_file(arg: Option<String>) {
    unimplemented!("run_file fn is not implemented yet.");
}

fn run_prompt() {
    unimplemented!("run_prompt fn is not implemented yet.");
}
