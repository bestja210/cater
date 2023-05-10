use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>, // files will be of form vector contianing strings
    number_lines: bool, // indicates user pref for numbered lines or not
    number_nonblank_lines: bool,  // indicates printing numbers only for nonblank lines
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cater")
        .version("0.1.0")
        .author("Jacob S. Best <bestja210@gmail.com>")
        .about("Rust Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true) // Required to have atleast 1 value or defaults to false.
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Numbers All Lines That Are Not Blank")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files { // iterate through each filename
        match open(&filename) { // try to open the BORROWED file
            Err(e) => eprintln!("{}: {}", filename, e), // Print error message to STDERR if open fails.
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() { // enumerate creates a tuple which we unpack using pattern matching
                    let line = line_result?; // either unpack an Ok(()) result or propogate an error.
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line)
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> { // Accepts a filename and returns either an Error or a Boxed value that implements the Boxed trait.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // When file is - read from std::io::stdin
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // Use File::open to read file or propogate error. If open is successful result will be filehandle
    }
}
