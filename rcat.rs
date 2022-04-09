use std::{env, process::exit};
use std::path::PathBuf;
use std::io::{self, Read};
const HELP: &str = "Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

-A, --show-all
       equivalent to -vET

-b, --number-nonblank
       number nonempty output lines, overrides -n

-e     equivalent to -vE

-E, --show-ends
       display $ at end of each line

-n, --number
       number all output lines

-s, --squeeze-blank
       suppress repeated empty output lines

-t     equivalent to -vT

-T, --show-tabs
       display TAB characters as ^I

-u     (ignored)

-v, --show-nonprinting
       use ^ and M- notation, except for LFD and TAB

--help display this help and exit

--version
       output version information and exit";
struct Flags{
    number_nonblank: bool,
    show_ends: bool,
    number: bool,
    squeeze_blank: bool,
    show_tabs: bool,
    show_nonprinting: bool,
}
fn main() {
    let mut flags: Flags = Flags { number_nonblank: false, show_ends: false, number: false, squeeze_blank: false, show_tabs: false, show_nonprinting: false };
    let mut args: Vec<String> = env::args().collect();
    let mut read_files: Vec<String> = [].to_vec();
    let mut concat_string: String = "".to_string();
    let current_path: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    args.remove(0);
    if args.is_empty() {
        exit(0);
    }

    for arg in args.iter().enumerate() {
        if arg.1.starts_with('-') {
            match arg.1.as_str() {
                "--version" => {println!("rcat 1.0.0"); exit(0)},
                "--help" => {println!("{}",HELP); exit(0)},
                "-" => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer).unwrap_or_default();
                    read_files.push(buffer);
                }
                _ => {
                    let mut arg_chars: Vec<char> = arg.1.chars().collect();
                    arg_chars.remove(0);
                    for char in arg_chars {
                        match char {
                            'A' => {
                                flags.show_nonprinting = true;
                                flags.show_ends = true;
                                flags.show_tabs = true;
                            },
                            'b' => flags.number_nonblank = true,
                            'e' => {
                                flags.show_nonprinting = true;
                                flags.show_ends = true;
                            },
                            'E' => flags.show_ends = true,
                            'n' => flags.number = true,
                            's' => flags.squeeze_blank = true,
                            't' => {
                                flags.show_nonprinting = true;
                                flags.show_tabs = true;
                            },
                            'T' => flags.show_tabs = true,
                            'u' => TODO(char),
                            'v' => flags.show_nonprinting = true,
                            _ => { eprintln!("{} is not a valid flag",char)},

                        }
                    }
                }
                //{println!("Argument {} is not a valid argument",&arg.1); exit(1)},
            }
        } else {
            let path: PathBuf = [&current_path,arg.1].iter().collect();
            let temp = std::fs::read_to_string(path).unwrap_or_default();
            if !temp.is_empty() {
                read_files.push(temp);
            }
        }
    }

    for file in &read_files {
        concat_string.push_str(&file);
    }

    if flags.squeeze_blank {
        concat_string = concat_string.replace("\r\n\r\n", "\n\n").replace("\n\n", "\n");
    }

    if flags.number {
        let temp = concat_string.clone();
        let temp_lines = temp.lines();
        concat_string.clear();
        for lines in temp_lines.enumerate() {
            let temp_str = format!("{} {}\n",&lines.0.to_string(),lines.1);
            concat_string.push_str(&temp_str);
        }
    }

    if flags.number_nonblank {
        concat_string = concat_string.replace('\t', "^I");
    }

    if flags.show_ends {
        concat_string = concat_string.replace("\r\n","\n").replace("\n", "$\n");
        if !concat_string.ends_with('\n') {
            concat_string.push_str("$");
        }
    }
    print!("{}",concat_string);
}

fn TODO(X: char) {
    println!("{} is not implemented yet",X);
}
