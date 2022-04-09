use std::{env, process::exit};
use std::path::PathBuf;
const HELP: &str = "Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

-A, --show-all
       equivalent to -vET (PARTIALLY IMPLEMENTED)

-b, --number-nonblank
       number nonempty output lines, overrides -n

-e     equivalent to -vE (PARTIALLY IMPLEMENTED)

-E, --show-ends
       display $ at end of each line

-n, --number
       number all output lines

-s, --squeeze-blank
       suppress repeated empty output lines

-t     equivalent to -vT (NOT IMPLEMENTED)

-T, --show-tabs
       display TAB characters as ^I

-u     (ignored)

-v, --show-nonprinting
       use ^ and M- notation, except for LFD and TAB (NOT IMPLEMENTED)

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
    ignored: bool,
}
fn main() {
    let mut flags: Flags = Flags { number_nonblank: false, show_ends: false, number: false, squeeze_blank: false, show_tabs: false, show_nonprinting: false, ignored: false};
    let mut args: Vec<String> = env::args().collect();
    let mut read_files: Vec<String> = [].to_vec();
    let mut concat_string: String = "".to_string();
    let current_path: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();

    args.remove(0);
    if args.is_empty(){
            let mut input = String::new();
            std::io::stdin().read_line(& mut input).expect("Not empty");
            print!("{}", input);
    }

    for arg in args.iter().enumerate() {
        if arg.1.starts_with('-') {
            match arg.1.as_str() {
                "--version" => {println!("rcat - concatenate files together\nThis a cat reimplentation is Rust to learn the language and should not be used seriously\nEspecially considering probably a million other people have done exactly the same\nAlso just use the normal version, not everything has to be written in Rust"); exit(0)},
                "--help" => {println!("{}",HELP); exit(0)},
                "-" => {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
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
                            'u' => flags.ignored = true,
                            'v' => flags.show_nonprinting = true,
                            _ => { eprintln!("{} is not a valid flag\nCheck the --help flag for more info",char); exit(1)},

                        }
                    }
                }            }
        } else {
            let path: PathBuf = [&current_path,arg.1].iter().collect();
            let temp = std::fs::read_to_string(path).unwrap_or_default();
            if !temp.is_empty() {
                read_files.push(temp);
            } else {
                println!("{}: No such file or directory",arg.1);
            }
        }
    }

    for file in &read_files {
        concat_string.push_str(&file);
    }

    if flags.squeeze_blank {
        while concat_string.contains("\r\n\r\n\r\n")||concat_string.contains("\n\n\n") {
            concat_string = concat_string.replace("\r\n\r\n\r\n\r\n", "\n\n\n\n").replace("\n\n\n\n", "\n\n").replace("\n\n\n", "\n\n");
        } 
    }

    if flags.number || flags.number_nonblank {
        flags.number = if flags.number_nonblank {false} else {true};
        let temp = concat_string.clone();
        let temp_lines = temp.lines();
        let mut line_offset: i32 = 0;
        concat_string.clear();
        for lines in temp_lines.enumerate() {
            let mut line_count: i32 = 0;
            if flags.number_nonblank && lines.1.is_empty() {
                line_offset += 1; 
            } 
            else {
                line_count = lines.0 as i32;
            };
            let temp_str = format!("     {}  {}\n",if !lines.1.is_empty()||flags.number {(line_count-line_offset).to_string()} else {"".to_string()},lines.1);
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
    exit(0);
}