extern crate clap;
mod tape;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};
use tape::Tape;

fn main() {
    let args = App::new("brainfuck interpreter")
        .version("1.0")
        .author("Michael Cordover <bf@mjec.net>")
        .about("Runs a brainfuck program, where EOF returns 0x00 and the tape is an unlimited sequence of bytes.")
        .arg(
            Arg::with_name("FILE")
                .index(1)
                .help("The file containing a brainfuck program to run")
                .required(true),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Write the state of the tape at every tick to stderr"),
        )
        .get_matches();

    let mut f = File::open(args.value_of("FILE").unwrap()).expect("Unable to read input file");
    let mut program: Vec<u8> = Vec::new();
    f.read_to_end(&mut program)
        .expect("Unable to read input file");

    let mut tape = Tape::new();
    let mut i = 0;

    loop {
        if i >= program.len() {
            // End of program
            break;
        }

        let byte = program[i];

        if args.is_present("debug") {
            eprintln!("{} {:?}", program[i] as char, tape);
        }

        /*
        Brainfuck consists of the following eight commands:
            >	Move the pointer to the right
            <	Move the pointer to the left
            +	Increment the memory cell under the pointer
            -	Decrement the memory cell under the pointer
            .	Output the character signified by the cell at the pointer
            ,	Input a character and store it in the cell at the pointer
            [	Jump past the matching ] if the cell under the pointer is 0
            ]	Jump back to the matching [ if the cell under the pointer is nonzero
        All other characters are considred comments and ignored.
        Note that , on EOF will use the value 0.
        */
        match byte {
            b'>' => tape.move_pointer_by(1),
            b'<' => tape.move_pointer_by(-1),
            b'+' => tape.increment_value(1),
            b'-' => tape.decrement_value(1),
            b'.' => print!("{}", tape.get_value() as char),
            b',' => tape.set_value(get_char_from_stdin().unwrap_or(0)),
            b'[' => {
                if tape.get_value() == 0 {
                    let starting_bracket = i;
                    let mut level = 0;
                    loop {
                        i += 1;
                        if i >= program.len() {
                            exit_with_error(
                                &format!(
                                    "Syntax error! Unmatched bracket '[' in input at byte {}.",
                                    starting_bracket
                                ),
                                2,
                            );
                        }
                        if program[i] == b']' {
                            if level == 0 {
                                i += 1;
                                break;
                            } else {
                                level -= 1;
                            }
                        }
                        if program[i] == b'[' {
                            level += 1;
                        }
                    }
                }
            }
            b']' => {
                if tape.get_value() != 0 {
                    let starting_bracket = i;
                    let mut level = 0;
                    loop {
                        if i == 0 {
                            exit_with_error(
                                &format!(
                                    "Syntax error! Unmatched bracket ']' in input at byte {}.",
                                    starting_bracket
                                ),
                                2,
                            );
                        }
                        i -= 1;
                        if program[i] == b'[' {
                            if level == 0 {
                                break;
                            } else {
                                level -= 1;
                            }
                        }
                        if program[i] == b']' {
                            level += 1;
                        }
                    }
                }
            }
            _ => (),
        };
        i += 1;
    }
}

fn get_char_from_stdin() -> Option<u8> {
    let mut buf: [u8; 1] = [0u8];
    match io::stdin().read_exact(&mut buf) {
        Ok(_) => Some(buf[0]),
        _ => None,
    }
}

fn exit_with_error(error: &str, error_code: i32) {
    println!("{}", error);
    ::std::process::exit(error_code);
}
