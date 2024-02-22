use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_lines_words_chars_bytes<R: BufRead>(reader: R) -> (usize, usize, usize, usize) {
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut byte_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            line_count += 1;
            let bytes = line.as_bytes().len();
            byte_count += bytes;
            word_count += line.split_whitespace().count();
            char_count += line.chars().count();
        }
    }

    (line_count, word_count, char_count, byte_count)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_file = "text.txt";

    if args.len() == 0 || args.len() > 3 {
        println!("Usage: {} <option> <file>", args[0]);
        println!("Options: -b, -l, -w, -c");
        return;
    }

    let file_path = if args.len() == 3 {
        &args[2]
    } else {
        default_file
    };

    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let (lines, words, chars, bytes) = count_lines_words_chars_bytes(reader);

            if args.len() == 1 {
                println!("Lines: {}", lines);
                println!("Words: {}", words);
                println!("Chars: {}", chars);
                println!("Bytes: {}", bytes);
            }

            if args.len() == 3 {
                match args[1].as_str() {
                    "-l" => println!("Lines: {}", lines),
                    "-w" => println!("Words: {}", words),
                    "-c" => println!("Chars: {}", chars),
                    "-b" => println!("Bytes: {}", bytes),
                    _ => println!("Usage: {} <option> <file>", args[0]),
                }
            }

            if args.len() == 2 {
                match args.len() {
                    2 => match args[1].as_str() {
                        "-l" => println!("Lines: {}", lines),
                        "-w" => println!("Words: {}", words),
                        "-c" => println!("Chars: {}", chars),
                        "-b" => println!("Bytes: {}", bytes),
                        _ => println!("Invalid option"),
                    },
                    _ => {
                        println!("Lines: {}", lines);
                        println!("Words: {}", words);
                        println!("Chars: {}", chars);
                        println!("Bytes: {}", bytes);
                    }
                }
            }
        }
        Err(_) => eprintln!("wc: {}: No such file or directory", file_path),
    }
}
