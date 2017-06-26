#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use docopt::Docopt;

mod lines_args;
use lines_args::*;

const USAGE: &'static str = "
Usage: lines [options] <file> <start> <stop>
       lines (--help | --version)

Description:
    Prints only said section from the file, ranging [start,stop] (inclusive at
    both ends).

Options:
    -h, --help      Display this help and exits
    --version       Display version information
    -l, --lines     Display corresponding line number
    -s, --spaces S  Padding for line number
";

fn main() {
    let raw_args: Vec<_> = env::args().collect();
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.argv(raw_args.into_iter()).deserialize())
                        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{} v{}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("Author:");
        println!("{}", env!("CARGO_PKG_AUTHORS"));
        return;
    }

    let args = check_args(args).unwrap();

    let f = File::open(&args.arg_file).unwrap();
    let file = BufReader::new(&f);
    let mut index: u64 = 1;

    for line in file.lines() {
        if index >= args.arg_start && index <= args.arg_stop {
            if args.flag_lines {
                // Print line number
                let max_len = match args.flag_spaces {
                    None    => args.arg_stop.to_string().len(),
                    Some(n) => n
                };
                println!("{:01$}:{2}", index, max_len, line.unwrap());
            } else {
                println!("{}", line.unwrap());
            }
        }
        if index > args.arg_stop {
            break;
        }
        index += 1;
    }
}
