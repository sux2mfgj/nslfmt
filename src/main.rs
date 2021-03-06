extern crate backtrace;
extern crate getopts;

use backtrace::Backtrace;
use getopts::Options;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::panic;
use std::process;

mod ast;
mod generator;
mod lexer;
mod parser;
mod token;

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

fn print_version() {
    let version_info = format!(
        "{} ({})",
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        include_str!(concat!(env!("OUT_DIR"), "/build-date.txt"))
    );

    println!("nslfmt {}", version_info);
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: nslfmt FILE [options]");
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version");
    opts.optflag("d", "debug", "print debug info");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(opts);
        process::exit(-1);
    }
    if matches.opt_present("v") {
        print_version();
        process::exit(0);
    }

    if matches.opt_present("d") {
        panic::set_hook(Box::new(|_| {
            let bt = Backtrace::new();
            eprintln!("{:?}", bt);
        }));
    }

    let input_file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(opts);
        process::exit(-1);
    };

    let fd = match File::open(input_file) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            process::exit(-1);
        }
    };

    let mut b = BufReader::new(fd);
    let mut l = Lexer::new(&mut b);

    let p = Parser::new(&mut l);
    let mut io = std::io::stdout();

    {
        let mut g = Generator::new(p, &mut io);
        match g.output_node() {
            Ok(()) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
